use super::{strokes, RenderState, SurfaceId};
use crate::shapes::{Shadow, Shape, Stroke, Type};
use skia_safe::{self as skia, Paint};

// Drop Shadows
pub fn render_drop_shadows(render_state: &mut RenderState, shape: &Shape) {
    if shape.has_fills() {
        for shadow in shape.drop_shadows().rev().filter(|s| !s.hidden()) {
            render_fill_drop_shadow(render_state, &shape, &shadow);
        }
    } else {
        // let scale = render_state.get_scale();
        for shadow in shape.drop_shadows().rev().filter(|s: &&Shadow| !s.hidden()) {
            render_stroke_drop_shadows(render_state, &shadow, &shape);
        }
    }
}

fn render_fill_drop_shadow(render_state: &mut RenderState, shape: &Shape, shadow: &Shadow) {
    let paint = &shadow.get_drop_shadow_paint();
    render_shadow_paint(render_state, shape, paint, SurfaceId::DropShadows);
}

pub fn render_stroke_drop_shadows(render_state: &mut RenderState, shadow: &Shadow, shape: &Shape) {
    let paint: &_ = &shadow.get_drop_shadow_paint();
    render_border_shadow_paint(render_state, shape, paint, SurfaceId::DropShadows);
}

// Inner Shadows
pub fn render_inner_shadows(render_state: &mut RenderState, shape: &Shape) {
    if shape.has_fills() {
        for shadow in shape.inner_shadows().rev().filter(|s| !s.hidden()) {
            render_fill_inner_shadow(render_state, &shape, &shadow);
        }
    } else {
        let scale = render_state.get_scale();
        for shadow in shape.inner_shadows().rev().filter(|s| !s.hidden()) {
            render_stroke_inner_shadow(render_state, &shadow, scale);
        }
    }
}

fn render_fill_inner_shadow(render_state: &mut RenderState, shape: &Shape, shadow: &Shadow) {
    let paint = &shadow.get_inner_shadow_paint();
    render_shadow_paint(render_state, shape, paint, SurfaceId::InnerShadows);
}

// TODO: Stroke shadows
fn render_stroke_inner_shadow(render_state: &mut RenderState, shadow: &Shadow, scale: f32) {
    let shadow_paint = &shadow.to_paint(scale);

    render_state
        .surfaces
        .draw_into(SurfaceId::Strokes, SurfaceId::Shadow, Some(shadow_paint));

    render_state.surfaces.draw_into(
        SurfaceId::Shadow,
        SurfaceId::Overlay,
        Some(&skia::Paint::default()),
    );

    render_state
        .surfaces
        .canvas(SurfaceId::Shadow)
        .clear(skia::Color::TRANSPARENT);
}

fn render_shadow_paint(
    render_state: &mut RenderState,
    shape: &Shape,
    paint: &Paint,
    surface_id: SurfaceId,
) {
    match &shape.shape_type {
        Type::Rect(_) | Type::Frame(_) => {
            render_state.surfaces.draw_rect_to(surface_id, shape, paint);
        }
        Type::Circle => {
            render_state
                .surfaces
                .draw_circle_to(surface_id, shape, paint);
        }
        Type::Path(_) | Type::Bool(_) => {
            render_state.surfaces.draw_path_to(surface_id, shape, paint);
        }
        _ => {}
    }
}

fn render_border_shadow_paint(
    render_state: &mut RenderState,
    shape: &Shape,
    paint: &Paint,
    surface_id: SurfaceId,
) {
    match &shape.shape_type {
        Type::Rect(_) | Type::Frame(_) => {
            println!("Not implemented");
        }
        Type::Circle => {
            println!("Not implemented");
        }
        Type::Path(_) | Type::Bool(_) => {
            for stroke in shape.strokes() {
                if let Some(path) = shape.shape_type.path() {
                    // let stroke_paint = stroke.to_stroked_paint(path.is_open(), &shape.selrect, &shape.svg_attrs, 1.0);
                    render_state
                        .surfaces
                        .draw_border_path_to(surface_id, &shape, &paint, &stroke);
                }
            }
        }
        _ => {}
    }
}
