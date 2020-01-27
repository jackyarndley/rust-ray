![rust-ray](output.png)
# rust-ray
Rust implementation of a simple ray tracer based off the book 'Ray Tracing in One Weekend' by Peter Shirley. 

Several additional features have been added to the basic ray-tracer presented in the book. Emmissive materials have been added in addition to the Dielectric, Metallic and Lambertian materials, which allows for the scene to be lit solely by user defined light sources if required. Furthermore, ray-triangle intersections are now possible, and therefore custom meshes are able to be imported into the ray-tracer, courtesy of tobj. Finally, the code has been parallelized with help of rayon to use all available CPU performance whilst rendering.
