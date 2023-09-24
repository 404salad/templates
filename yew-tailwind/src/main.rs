mod app; use app::App;
mod header; use header::Header;

pub struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

fn main() {
    yew::Renderer::<Header>::new().render();
    yew::Renderer::<App>::new().render();
}
