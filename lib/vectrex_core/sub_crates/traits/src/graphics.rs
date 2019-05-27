
pub struct PassId(pub u32);

pub struct MeshId(pub u32);
pub struct ShaderId(pub u32);

pub struct CameraId(pub u32);



pub struct PassInfo
{
    id: PassId,
    shader: ShaderId,
    mesh: MeshId
}



pub struct DrawRequest
{
    pub passes: Vec<PassInfo>
}



pub trait Renderer
{
    fn name(&self) -> String;
    fn info(&self) -> String;

    fn new_pass(&mut self, camera: CameraId) -> PassId;

    fn submit(&mut self, draw_request: &DrawRequest);

    fn resize_view(&self, width: i32, height: i32);
    fn render(&self);
}



pub trait RenderHandler
{
    fn submit(&self, renderer: &mut Renderer);
}
