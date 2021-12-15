A simple Monogame inspired engine that targets Windows, Linux, Mac and the Web. 

# Roadmap
- [x] Client + Engine built
- [x] Add in windowing
- [x] Add in an abstracted file system
- [x] Add in web target build script
- [x] Add web targets
- [x] build and run on all targets
- [x] Add in React client and Yarn 
- [x] Get web target served
- [x] Serve and execute WASM https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
- [ ] Clean up project so that the engine is split out from everything else?
- [ ] Make engine `no-std`? Gotta figure out the issues with Instant and the like.
- [ ] Add time module and get working on native and web
- [ ] Add in drawing of triangles for mac, linux, web, windows
- [ ] Add drawing of images
- [ ] Get file system running for mac, linux, web, windows
- [ ] Add in text renderer
- [ ] Add in sound?
- [ ] Add in script to preprocess things, like for web ensure that all audio is highly compressed, all images, etc.


# Renderer Roadmap
The renderer should have a clear separation between the GPU and the software rendering. The GPU should be kept as simple and as stupid as possible to ensure maximum compatibility. The only things that will be allowed are images. Polygons may make a showing later on, but I think I can get really far with just images. All transforms will be done in the CPU so I don't have to worry about learning more than simple image drawing on the GPU. No rotations at this point. This should make it easier to get web, pc, mobile, etc. up and running with minimal platform specific code. 
- [ ] Come up with a high level interface
- [ ] Stub out queueing system
- [ ] Wire up OpenGL 
- [ ] Wire up WGPU

