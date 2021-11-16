# arith
README:

Michael Pereira, Hunter Larkin

-Used the professor’s “Array2” implementation in our project. Consulted TA’s at multiple points in the assignment. Briefly consulted with other students to discuss possible design choices. 

-Every major component of the assignment was implemented correctly, as per the instructions.

-The overall architecture of our program goes as follows for compression: take the input image, transform rgb values to floating point, transform rgb values to component video, perform DCT and pack into a 32 bit word. To decompress, follow the same steps in reverse. The compressing transformation is handled in its own crate, as is the decompressing transformation. Each major step outlined above is being completed with their own separate functions. “Array2” and “bitpack” were separate modules from the “rpeg” module. Functions in “bitpack” were implemented using their own code, not by implementing signed functions for unsigned functions, or vice-versa.  

-Approximate amount of time spent analyzing the problems posed in this assignment: 3-4 hours.

-Approximate amount of time spent solving the problem: 25-30 hours.

