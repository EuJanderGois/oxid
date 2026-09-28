---
title: API Reference
slug: /api
---

# API reference

This section documents the current JavaScript surface exposed by the Oxid runtime.

The reference is intentionally conservative and follows the code that exists today. If an API is not documented here, treat it as not part of the current public runtime surface.

For how this API is exposed under the hood — the path from a Rust plugin to a callable JavaScript function, and how `oxid.d.ts` is generated from the same metadata described here — see [Architecture](/architecture).

## Conventions

A few conventions hold across every module below; they are called out here once instead of repeated on every page.

- **Coordinates**: screen space, origin `(0, 0)` at the top-left corner, `x` increasing right and `y` increasing down. This is Macroquad's convention, and Oxid does not currently transform it.
- **Colors**: `Color` components (`r`, `g`, `b`, `a`) are floating-point values, normally in the `0.0`–`1.0` range.
- **Rotation units are not uniform across modules.** [`oxid/shapes`](/api/shapes) (`drawArc`, `drawPolygonLines`) takes rotation **in degrees**; [`oxid/texture`](/api/texture) (`drawTextureScaled`) takes rotation **in radians**. This reflects a real difference in the native calls each module wraps — see the note on the [shapes](/api/shapes#rotation-units-degrees-here-radians-in-oxidtexture) page for detail. Always check the specific function's docs rather than assuming one convention.
- **Text baseline**: `drawText`/`drawMultilineText` treat the given `y` coordinate as the text baseline, not the top of the glyph box (see [`oxid/text`](/api/text)).
- **Draw functions only have an effect during `onDraw()`.** Every drawing function in `oxid/shapes`, `oxid/text`, and `oxid/texture` appends a command to the current frame's render queue; outside of the `onDraw()` call, there is no active queue, so the call is a silent no-op rather than an error.
