mod color;
mod drawing_application;
mod drawing_factory;
mod shape;

fn main() {
    let drawing_type = "Geometric";
    let factory: Box<dyn drawing_factory::DrawingFactory> = match drawing_type {
        "Geometric" => Box::new(drawing_factory::GeometricFactory),
        "Artistic" => Box::new(drawing_factory::ArtisticFactory),
        _ => panic!("Unknown drawing type."),
    };

    let mut app = drawing_application::DrawingApplication::new(factory);
    app.render();
}
