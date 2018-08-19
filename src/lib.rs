#![allow(unused_variables)]

// Ported to Rust 2018 by Joshua Cearley joshua.cearley@gmail.com
// Copyright (c) 2013 Mikko Mononen memon@inside.org
//
// This software is provided 'as-is', without any express or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.
//

//! # Transforms
//!
//! The paths, gradients, patterns and scissor region are transformed by an transformation
//! matrix at the time when they are passed to the API.
//!
//! The current transformation matrix is a affine matrix:
//!   [sx kx tx]
//!   [ky sy ty]
//!   [ 0  0  1]
//!
//! Where: sx,sy define scaling, kx,ky skewing, and tx,ty translation.
//! The last row is assumed to be 0,0,1 and is not stored.
//!
//! Apart from nvgResetTransform(), each transformation function first creates
//! specific transformation matrix and pre-multiplies the current transformation by it.
//!
//! Current coordinate system (transformation) can be saved and restored using nvgSave() and nvgRestore().
//!
//! # Paints
//!
//! NanoVG supports four types of paints: linear gradient, box gradient, radial gradient and image pattern.
//! These can be used as paints for strokes and fills.
//!
//! # Composite operation
//!
//! The composite operations in NanoVG are modeled after HTML
//! Canvas API, and the blend func is based on OpenGL (see
//! corresponding manuals for more info).  The colors in the
//! blending state have premultiplied alpha.
//!
//! # State Handling
//!
//! NanoVG contains state which represents how paths will be
//! rendered.  The state contains transform, fill and stroke
//! styles, text and font styles, and scissor clipping.
//!
//! # Render styles
//!
//! Fill and stroke render style can be either a solid color or a paint which is a gradient or a pattern.
//! Solid color is simply defined as a color value, different kinds of paints can be created
//! using nvgLinearGradient(), nvgBoxGradient(), nvgRadialGradient() and nvgImagePattern().
//!
//! Current render style can be saved and restored using nvgSave() and nvgRestore().
//!
//! # Images
//!
//! NanoVG allows you to load jpg, png, psd, tga, pic and gif files to be used for rendering.
//! In addition you can upload your own image. The image loading is provided by stb_image.
//! The parameter imageFlags is combination of flags defined in NVGimageFlags.
//!
//! # Scissoring
//!
//! Scissoring allows you to clip the rendering into a rectangle. This is useful for various
//! user interface cases like rendering a text edit or a timeline.
//!
//! # Paths
//!
//! Drawing a new shape starts with nvgBeginPath(), it clears all the currently defined paths.
//! Then you define one or more paths and sub-paths which describe the shape. The are functions
//! to draw common shapes like rectangles and circles, and lower level step-by-step functions,
//! which allow to define a path curve by curve.
//!
//! NanoVG uses even-odd fill rule to draw the shapes. Solid shapes should have counter clockwise
//! winding and holes should have counter clockwise order. To specify winding of a path you can
//! call nvgPathWinding(). This is useful especially for the common shapes, which are drawn CCW.
//!
//! Finally you can fill the path using current fill style by calling nvgFill(), and stroke it
//! with current stroke style by calling nvgStroke().
//!
//! The curve segments and sub-paths are transformed by the current transform.
//!
//! # Text
//!
//! NanoVG allows you to load .ttf files and use the font to render text.
//!
//! The appearance of the text can be defined by setting the current text style
//! and by specifying the fill color. Common text and font settings such as
//! font size, letter spacing and text align are supported. Font blur allows you
//! to create simple text effects such as drop shadows.
//!
//! At render time the font face can be set based on the font handles or name.
//!
//! Font measure functions return values in local space, the calculations are
//! carried in the same resolution as the final rendering. This is done because
//! the text glyph positions are snapped to the nearest pixels sharp rendering.
//!
//! The local space means that values are not rotated or scale as per the current
//! transformation. For example if you set font size to 12, which would mean that
//! line height is 16, then regardless of the current scaling and rotation, the
//! returned line height is always 16. Some measures may vary because of the scaling
//! since aforementioned pixel snapping.
//!
//! While this may sound a little odd, the setup allows you to always render the
//! same way regardless of scaling. I.e. following works regardless of scaling:
//!
//!		const char* txt = "Text me up.";
//!		nvgTextBounds(vg, x,y, txt, NULL, bounds);
//!		nvgBeginPath(vg);
//!		nvgRoundedRect(vg, bounds[0],bounds[1], bounds[2]-bounds[0], bounds[3]-bounds[1]);
//!		nvgFill(vg);
//!
//! Note: currently only solid color fill is supported for text.

#[inline(always)]
/// Converts degrees to radians.
pub fn deg_to_rad(deg: f32) -> f32 {
    unimplemented!();
}

#[inline(always)]
/// Converts radians to degrees.
pub fn rad_to_deg(rad: f32) -> f32 {
    unimplemented!();
}

pub mod math;

use math::*;

pub mod color;

use color::*;

pub struct Paint {
    pub xform:       [f32; 6],
    pub extent:      [f32; 2],
    pub radius:      f32,
    pub feather:     f32,
    pub inner_color: Color,
    pub outer_color: Color,
    pub image:       isize,
}

pub enum Winding {
    /// Winding for solid shapes.
    CCW,
    /// Winding for holes.
    CW,
}

pub enum Solidity {
    /// Counter clockwise.
    Solid,
    /// Clockwise.
    Hole,
}

pub enum LineCap {
    Butt,
    Round,
    Square,
}

pub enum LineJoin {
    Round,
    Bevel,
    Miter,
}

pub enum Align {
    // Horizontal align
    /// Default, align text horizontally to left.
    Left     = 1 << 0,
    /// Align text horizontally to center.
    Center   = 1 << 1,
    /// Align text horizontally to right.
    Right    = 1 << 2,

    // Vertical align
    /// Align text vertically to top.
    Top      = 1 << 3,
    /// Align text vertically to middle.
    Middle   = 1 << 4,
    /// Align text vertically to bottom.
    Bottom   = 1 << 5,
    /// Default, align text vertically to baseline.
    Baseline = 1 << 6,
}

pub enum BlendFactor {
    Zero                     = 1 << 0,
    One                      = 1 << 1,
    SourceColor              = 1 << 2,
    OneMinusSourceColor      = 1 << 3,
    DestinationColor         = 1 << 4,
    OneMinusDestinationColor = 1 << 5,
    SourceAlpha              = 1 << 6,
    OneMinusSourceAlpha      = 1 << 7,
    DestinationAlpha         = 1 << 8,
    OneMinusDestinationAlpha = 1 << 9,
    SourceAlphaSaturate      = 1 << 10,
}

pub enum CompositeOperation {
    SourceOver,
    SourceIn,
    SourceOut,
    Atop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Lighter,
    Copy,
    Xor,
}

pub struct CompositeOperationState {
    pub src_rgb:   isize,
    pub dst_rgb:   isize,
    pub src_alpha: isize,
    pub dst_alpha: isize,
}

pub struct GlyphPosition<'a> {
    /// Position of the glyph in the input string.
    pub glyph: &'a str,
    /// The x-coordinate of the logical glyph position.
    pub x:     f32,
    /// The bounds of the glyph shape.
    pub minx:  f32,
    /// The bounds of the glyph shape.
    pub maxx:  f32,
}

pub struct TextRow<'a> {
    /// Text from the input string.
    pub span: &'a str,
    /// Logical width of the row.
    pub width:    f32,
    /// Actual bounds of the row. Logical width and bounds can differ because of kerning and some parts over extending.
    pub minx:     f32,
    pub maxx:     f32,
}

pub enum ImageFlags {
    /// Generate mipmaps during creation of the image.
    GenerateMipmaps = 1 << 0,
    /// Repeat image in X direction.
    Repeatx         = 1 << 1,
    /// Repeat image in Y direction.
    Repeaty         = 1 << 2,
    /// Flips (inverses) image in Y direction when rendered.
    Flipy           = 1 << 3,
    /// Image data has premultiplied alpha.
    Premultiplied   = 1 << 4,
    /// Image interpolation is Nearest instead Linear
    Nearest         = 1 << 5,
}

pub trait Context {
    /// Begin drawing a new frame.
    ///
    /// Calls to nanovg drawing API should be wrapped in
    /// nvgBeginFrame() & nvgEndFrame() nvgBeginFrame() defines
    /// the size of the window to render to in relation currently
    /// set viewport (i.e. glViewport on GL backends). Device pixel
    /// ration allows to control the rendering on Hi-DPI devices.
    ///
    /// For example, GLFW returns two dimension for an opened window:
    /// window size and frame buffer size. In that case you would set
    /// windowWidth/Height to the window size devicePixelRatio to:
    /// frameBufferWidth / windowWidth.
    fn begin_frame(&mut self, window_width: f32, window_height: f32, device_pixel_ratio: f32);

    /// Cancels drawing the current frame.
    fn cancel_frame(&mut self);

    /// Ends drawing flushing remaining render state.
    fn end_frame(&mut self);

    /// Sets the composite operation.
    fn global_composite_operation(&mut self, op: CompositeOperation);

    /// Sets the composite operation with custom pixel arithmetic.
    fn global_composite_blend_func(&mut self, sfactor: BlendFactor, dfactor: BlendFactor);

    /// Sets the composite operation with custom pixel arithmetic for RGB and alpha components separately. The parameters should be one of NVGblendFactor.
    fn global_composite_blend_func_separate(&mut self, src_r_g_b: isize, dst_r_g_b: isize, src_alpha: isize, dst_alpha: isize);

    /// Pushes and saves the current render state into a state stack.
    /// A matching nvgRestore() must be used to restore the state.
    fn save(&mut self);

    /// Pops and restores current render state.
    fn restore(&mut self);

    /// Resets current render state to default values. Does not affect the render state stack.
    fn reset(&mut self);

    /// Sets whether to draw antialias for nvgStroke() and nvgFill(). It's enabled by default.
    fn shape_anti_alias(&mut self, enabled: bool);

    /// Sets current stroke style to a solid color.
    fn stroke_color(&mut self, color: Color);

    /// Sets current stroke style to a paint, which can be a one of the gradients or a pattern.
    fn stroke_paint(&mut self, paint: Paint);

    /// Sets current fill style to a solid color.
    fn fill_color(&mut self, color: Color);

    /// Sets current fill style to a paint, which can be a one of the gradients or a pattern.
    fn fill_paint(&mut self, paint: Paint);

    /// Sets the miter limit of the stroke style.
    /// Miter limit controls when a sharp corner is beveled.
    fn miter_limit(&mut self, limit: f32);

    /// Sets the stroke width of the stroke style.
    fn stroke_width(&mut self, size: f32);

    /// Sets how the end of the line (cap) is drawn,
    /// Can be one of: NVG_BUTT (default), NVG_ROUND, NVG_SQUARE.
    fn line_cap(&mut self, cap: LineCap);

    /// Sets how sharp path corners are drawn.
    /// Can be one of NVG_MITER (default), NVG_ROUND, NVG_BEVEL.
    fn line_join(&mut self, join: LineJoin);

    /// Sets the transparency applied to all rendered shapes.
    /// Already transparent paths will get proportionally more transparent as well.
    fn global_alpha(&mut self, alpha: f32);

    /// Resets current transform to a identity matrix.
    fn reset_transform(&mut self);

    /// Premultiplies current coordinate system by specified matrix.
    /// The parameters are interpreted as matrix as follows:
    ///   [a c e]
    ///   [b d f]
    ///   [0 0 1]
    fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32);

    /// Translates current coordinate system.
    fn translate(&mut self, x: f32, y: f32);

    /// Rotates current coordinate system. Angle is specified in radians.
    fn rotate(&mut self, angle: f32);

    /// Skews the current coordinate system along X axis. Angle is specified in radians.
    fn skew_x(&mut self, angle: f32);

    /// Skews the current coordinate system along Y axis. Angle is specified in radians.
    fn skew_y(&mut self, angle: f32);

    /// Scales the current coordinate system.
    fn scale(&mut self, x: f32, y: f32);

    /// Stores the top part (a-f) of the current transformation matrix in to the specified buffer.
    ///   [a c e]
    ///   [b d f]
    ///   [0 0 1]
    fn current_transform(&self) -> Transform;

    /// Creates image by loading it from the disk from specified file name.
    /// Returns handle to the image.
    fn create_image(&mut self, filename: &str, image_flags: isize) -> isize;

    /// Creates image by loading it from the specified chunk of memory.
    /// Returns handle to the image.
    fn create_image_mem(&mut self, image_flags: isize, data: &[u8]) -> isize;

    /// Creates image from specified image data.
    /// Returns handle to the image.
    fn create_image_r_g_b_a(&mut self, w: isize, h: isize, image_flags: isize, data: &[u8]) -> isize;

    /// Updates image data specified by image handle.
    fn update_image(&mut self, image: isize, data: &[u8]);

    /// Returns the width and height of a created image.
    fn image_size(&mut self, image: isize) -> (usize, usize);

    /// Deletes created image.
    fn delete_image(&mut self, image: isize);

    /// Creates and returns a linear gradient. Parameters (sx,sy)-(ex,ey) specify the start and end coordinates
    /// of the linear gradient, icol specifies the start color and ocol the end color.
    /// The gradient is transformed by the current transform when it is passed to nvgFillPaint() or nvgStrokePaint().
    fn linear_gradient(&mut self, sx: f32, sy: f32, ex: f32, ey: f32, icol: Color, ocol: Color) -> Paint;

    /// Creates and returns a box gradient. Box gradient is a feathered rounded rectangle, it is useful for rendering
    /// drop shadows or highlights for boxes. Parameters (x,y) define the top-left corner of the rectangle,
    /// (w,h) define the size of the rectangle, r defines the corner radius, and f feather. Feather defines how blurry
    /// the border of the rectangle is. Parameter icol specifies the inner color and ocol the outer color of the gradient.
    /// The gradient is transformed by the current transform when it is passed to nvgFillPaint() or nvgStrokePaint().
    fn box_gradient(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, f: f32, icol: Color, ocol: Color) -> Paint;

    /// Creates and returns a radial gradient. Parameters (cx,cy) specify the center, inr and outr specify
    /// the inner and outer radius of the gradient, icol specifies the start color and ocol the end color.
    /// The gradient is transformed by the current transform when it is passed to nvgFillPaint() or nvgStrokePaint().
    fn radial_gradient(&mut self, cx: f32, cy: f32, inr: f32, outr: f32, icol: Color, ocol: Color) -> Paint;

    /// Creates and returns an image pattern. Parameters (ox,oy) specify the left-top location of the image pattern,
    /// (ex,ey) the size of one image, angle rotation around the top-left corner, image is handle to the image to render.
    /// The gradient is transformed by the current transform when it is passed to nvgFillPaint() or nvgStrokePaint().
    fn image_pattern(&mut self, ox: f32, oy: f32, ex: f32, ey: f32, angle: f32, image: isize, alpha: f32) -> Paint;

    /// Sets the current scissor rectangle.
    /// The scissor rectangle is transformed by the current transform.
    fn scissor(&mut self, x: f32, y: f32, w: f32, h: f32);

    /// Intersects current scissor rectangle with the specified rectangle.
    /// The scissor rectangle is transformed by the current transform.
    /// Note: in case the rotation of previous scissor rect differs from
    /// the current one, the intersection will be done between the specified
    /// rectangle and the previous scissor rectangle transformed in the current
    /// transform space. The resulting shape is always rectangle.
    fn intersect_scissor(&mut self, x: f32, y: f32, w: f32, h: f32);

    /// Reset and disables scissoring.
    fn reset_scissor(&mut self);

    /// Clears the current path and sub-paths.
    fn begin_path(&mut self);

    /// Starts new sub-path with specified point as first point.
    fn move_to(&mut self, x: f32, y: f32);

    /// Adds line segment from the last point in the path to the specified point.
    fn line_to(&mut self, x: f32, y: f32);

    /// Adds cubic bezier segment from last point in the path via two control points to the specified point.
    fn bezier_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32);

    /// Adds quadratic bezier segment from last point in the path via a control point to the specified point.
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32);

    /// Adds an arc segment at the corner defined by the last path point, and two specified points.
    fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32);

    /// Closes current sub-path with a line segment.
    fn close_path(&mut self);

    /// Sets the current sub-path winding, see NVGwinding and NVGsolidity.
    fn path_winding(&mut self, dir: Winding);

    /// Creates new circle arc shaped sub-path. The arc center is at cx,cy, the arc radius is r,
    /// and the arc is drawn from angle a0 to a1, and swept in direction dir (NVG_CCW, or NVG_CW).
    /// Angles are specified in radians.
    fn arc(&mut self, cx: f32, cy: f32, r: f32, a0: f32, a1: f32, dir: Winding);

    /// Creates new rectangle shaped sub-path.
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32);

    /// Creates new rounded rectangle shaped sub-path.
    fn rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32);

    /// Creates new rounded rectangle shaped sub-path with varying radii for each corner.
    fn rounded_rect_varying(&mut self, x: f32, y: f32, w: f32, h: f32, rad_top_left: f32, rad_top_right: f32, rad_bottom_right: f32, rad_bottom_left: f32);

    /// Creates new ellipse shaped sub-path.
    fn ellipse(&mut self, cx: f32, cy: f32, rx: f32, ry: f32);

    /// Creates new circle shaped sub-path.
    fn circle(&mut self, cx: f32, cy: f32, r: f32);

    /// Fills the current path with current fill style.
    fn fill(&mut self);

    /// Fills the current path with current stroke style.
    fn stroke(&mut self);

    /// Creates font by loading it from the disk from specified file name.
    /// Returns handle to the font.
    fn create_font(&mut self, name: &str, filename: &str) -> isize;

    /// Creates font by loading it from the specified memory chunk.
    /// Returns handle to the font.
    fn create_font_mem(&mut self, name: &str, data: &[u8], ndata: isize, free_data: isize) -> isize;

    /// Finds a loaded font of specified name, and returns handle to it, or -1 if the font is not found.
    fn find_font(&mut self, name: &str) -> isize;

    /// Adds a fallback font by handle.
    fn add_fallback_font_id(&mut self, base_font: isize, fallback_font: isize) -> isize;

    /// Adds a fallback font by name.
    fn add_fallback_font(&mut self, base_font: &str, fallback_font: &str) -> isize;

    /// Sets the font size of current text style.
    fn font_size(&mut self, size: f32);

    /// Sets the blur of current text style.
    fn font_blur(&mut self, blur: f32);

    /// Sets the letter spacing of current text style.
    fn text_letter_spacing(&mut self, spacing: f32);

    /// Sets the proportional line height of current text style. The line height is specified as multiple of font size.
    fn text_line_height(&mut self, line_height: f32);

    /// Sets the text align of current text style, see NVGalign for options.
    fn text_align(&mut self, align: Align);

    /// Sets the font face based on specified id of current text style.
    fn font_face_id(&mut self, font: isize);

    /// Sets the font face based on specified name of current text style.
    fn font_face(&mut self, font: &str);

    /// Draws text string at specified location. If end is specified only the sub-string up to the end is drawn.
    fn text(&mut self, x: f32, y: f32, span: &str) -> f32;

    /// Draws multi-line text string at specified location wrapped at the specified width. If end is specified only the sub-string up to the end is drawn.
    /// White space is stripped at the beginning of the rows, the text is split at word boundaries or when new-line characters are encountered.
    /// Words longer than the max width are slit at nearest character (i.e. no hyphenation).
    fn text_box(&mut self, x: f32, y: f32, break_row_width: f32, span: &str);

    /// Measures the specified text string. Parameter bounds should be a pointer to float[4] -> f32;
    /// if the bounding box of the text should be returned. The bounds value are [xmin,ymin, xmax,ymax]
    /// Returns the horizontal advance of the measured text (i.e. where the next character should drawn).
    /// Measured values are returned in local coordinate space.
    fn text_bounds(&mut self, x: f32, y: f32, span: &str, bounds: &mut f32) -> f32;

    /// Measures the specified multi-text string. Parameter bounds should be a pointer to float[4],
    /// if the bounding box of the text should be returned. The bounds value are [xmin,ymin, xmax,ymax]
    /// Measured values are returned in local coordinate space.
    fn text_box_bounds(&mut self, x: f32, y: f32, break_row_width: f32, span: &str, bounds: &mut f32);

    /// Calculates the glyph x positions of the specified text. If end is specified only the sub-string will be used.
    /// Measured values are returned in local coordinate space.
    fn text_glyph_positions_into(&mut self, x: f32, y: f32, span: &str, vec: &mut Vec<GlyphPosition>);

    /// Returns the vertical metrics based on the current text style.
    /// Measured values are returned in local coordinate space.
    fn text_metrics(&mut self, ascender: &mut f32, descender: &mut f32, lineh: &mut f32);

    /// Breaks the specified text into lines. If end is specified only the sub-string will be used.
    /// White space is stripped at the beginning of the rows, the text is split at word boundaries or when new-line characters are encountered.
    /// Words longer than the max width are slit at nearest character (i.e. no hyphenation).
    fn text_break_lines(&mut self, span: &str, break_row_width: f32, rows: &mut Vec<TextRow>) -> isize;
}
