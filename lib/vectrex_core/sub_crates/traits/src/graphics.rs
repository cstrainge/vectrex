

pub trait Renderer
{
    fn name(&self) -> String;
    fn info(&self) -> String;
}


pub trait Renderable
{
    fn render(&self, renderer: &Renderer);
}
