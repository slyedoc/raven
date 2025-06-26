# Bevy_Slyedoc_Bvh

A Bevy Plugin for bounding volume hierarchy.

This project is just an experiment for my own enjoyment at the moment.

## Context

Allows creating `Blas` (bvh for mesh data) and `Tlas` (collection of blas's) which can be ray_cast agaist

| Features | Notes |
| -------- | ------- |
| `camera`   |  Addes `TlasCamera` which can let you visisual the tlas as image. **Debug Only**|
| `debug_draw` | Adddes `BvhDebugPlugin` for displaying gizmos |

## Credit

This is largely based on the amazing [tutorial series](https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/) by Jacco Bikker.  Go check it out if bvh's interest you.

And of course bevy and its community.

## Other Resources
- [tutorial series](https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/) by Jacco Bikker.
- [Ray Tracing in One Weekend](https://raytracing.github.io/) How everyone gets started with raytracing anymore.
- [Trace-Efficiency](https://www.nvidia.com/docs/IO/76976/HPG2009-Trace-Efficiency.pdf) Old NVidia paper exploring different ideas (Jacco Bikker tweet)


## License

License under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.