
//@HEADER
// ************************************************************************
// 
//               HPCCG: Simple Conjugate Gradient Benchmark Code
//                 Copyright (2006) Sandia Corporation
// 
// Under terms of Contract DE-AC04-94AL85000, there is a non-exclusive
// license for use of this work by or on behalf of the U.S. Government.
// 
// BSD 3-Clause License
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// 
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
// 
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
// 
// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
// 
// Questions? Contact Michael A. Heroux (maherou@sandia.gov) 
// 
// ************************************************************************
//@HEADER

/////////////////////////////////////////////////////////////////////////

// Routine to read a sparse matrix, right hand side, initial guess, 
// and exact solution (as computed by a direct solver).

/////////////////////////////////////////////////////////////////////////

// nrow - number of rows of matrix (on this processor)

#include <iostream>
using std::cout;
using std::cerr;
using std::endl;
#include <cstdlib>
#include <cstdio>
#include <cassert>
#include "generate_matrix.hpp"
void generate_matrix(int nx, int ny, int nz, HPC_Sparse_Matrix **A, double **x, double **b, double **xexact)

{
//#ifdef DEBUG
//  int debug = 1;
//#else
  int debug = 1;
//#endif


  *A = new HPC_Sparse_Matrix; // Allocate matrix struct and fill it
  (*A)->title = 0;


  // Set this bool to true if you want a 7-pt stencil instead of a 27 pt stencil
  bool use_7pt_stencil = false;

  int local_nrow = nx*ny*nz; // This is the size of our subblock
  assert(local_nrow>0); // Must have something to work with
  int local_nnz = 27*local_nrow; // Approximately 27 nonzeros per row (except for boundary nodes)
  int stop_row = local_nrow-1;

  // Allocate arrays that are of length local_nrow
  (*A)->nnz_in_row = new int[local_nrow];
  (*A)->ptr_to_vals_in_row = new double*[local_nrow];
  (*A)->ptr_to_inds_in_row = new int   *[local_nrow];
  (*A)->ptr_to_diags       = new double*[local_nrow];

  *x = new double[local_nrow];
  *b = new double[local_nrow];
  *xexact = new double[local_nrow];

  // Allocate arrays that are of length local_nnz
  (*A)->list_of_vals = new double[local_nnz];
  (*A)->list_of_inds = new int   [local_nnz];

  double * curvalptr = (*A)->list_of_vals;
  int * curindptr = (*A)->list_of_inds;

  long long nnzglobal = 0;
  for (int iz=0; iz<nz; iz++) {
    for (int iy=0; iy<ny; iy++) {
      for (int ix=0; ix<nx; ix++) {
	int curlocalrow = iz*nx*ny+iy*nx+ix;
	int currow = iz*nx*ny+iy*nx+ix;
	int nnzrow = 0;
	(*A)->ptr_to_vals_in_row[curlocalrow] = curvalptr;
	(*A)->ptr_to_inds_in_row[curlocalrow] = curindptr;
	for (int sz=-1; sz<=1; sz++) {
	  for (int sy=-1; sy<=1; sy++) {
	    for (int sx=-1; sx<=1; sx++) {
	      int curcol = currow+sz*nx*ny+sy*nx+sx;
//            Since we have a stack of nx by ny by nz domains , stacking in the z direction, we check to see
//            if sx and sy are reaching outside of the domain, while the check for the curcol being valid
//            is sufficient to check the z values
              if ((ix+sx>=0) && (ix+sx<nx) && (iy+sy>=0) && (iy+sy<ny) && (curcol>=0 && curcol<local_nrow)) {
                if (!use_7pt_stencil || (sz*sz+sy*sy+sx*sx<=1)) { // This logic will skip over point that are not part of a 7-pt stencil
                  if (curcol==currow) {
		    (*A)->ptr_to_diags[curlocalrow] = curvalptr;
		    *curvalptr++ = 27.0;
		  }
		  else {
		    *curvalptr++ = -1.0;
                  }
		  *curindptr++ = curcol;
		  nnzrow++;
	        } 
              }
	    } // end sx loop
          } // end sy loop
        } // end sz loop
	(*A)->nnz_in_row[curlocalrow] = nnzrow;
	nnzglobal += nnzrow;
	(*x)[curlocalrow] = 0.0;
	(*b)[curlocalrow] = 27.0 - ((double) (nnzrow-1));
	(*xexact)[curlocalrow] = 1.0;
      } // end ix loop
     } // end iy loop
  } // end iz loop

  (*A)->start_row = 0;
  (*A)->stop_row = stop_row;
  (*A)->total_nrow = local_nrow;
  (*A)->total_nnz = local_nnz;
  (*A)->local_nrow = local_nrow;
  (*A)->local_ncol = local_nrow;
  (*A)->local_nnz = local_nnz;

  if (debug) {
      cout << "Process "<<0<<" of "<<1<<" has "<<local_nrow;
      cout << " rows. Global rows "<< 0
           <<" through "<< stop_row <<endl;
      cout << "Process "<<0<<" of "<<1
           <<" has "<<local_nnz<<" nonzeros."<<endl<<endl;

      cout << "Start: " << 0 << " Stop: " << stop_row << " Total: " << local_nrow
           << " Total non-zeroes: " << local_nnz << endl;
      cout << "Local nrow: " << local_nrow << " Local ncol: " << local_nrow << " Local nnz" << local_nnz << endl;

      // nnz_in_row
      cout << "nnz_in_row (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << (*A)->nnz_in_row[i] << ", ";
      }
      cout << "]" << endl;
      // ptr_to_vals_in_row
      cout << "ptr_to_vals_in_row (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << *((*A)->ptr_to_vals_in_row[i]) << ", ";
      }
      cout << "]" << endl;
      // ptr_to_inds_in_row
      cout << "ptr_to_inds_in_row (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << *((*A)->ptr_to_inds_in_row[i]) << ", ";
      }
      cout << "]" << endl;
      // ptr_to_diags
      cout << "ptr_to_diags (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << *((*A)->ptr_to_diags[i]) << ", ";
      }
      cout << "]" << endl;

      // x
      cout << "x (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << (*x)[i] << ", ";
      }
      cout << "]" << endl;
      // b
      cout << "b (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << (*b)[i] << ", ";
      }
      cout << "]" << endl;
      // xexact
      cout << "xexact (" << stop_row+1 << "): [";
      for (int i=0; i<=stop_row; i++) {
          cout << (*xexact)[i] << ", ";
      }
      cout << "]" << endl;
  }

  return;
}
