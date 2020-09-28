#[derive(PartialEq,Debug)]
pub struct USD(f32); // we are not ritch so f32 will be enough
#[derive(PartialEq,Debug)]
pub struct GBP(f32); // Brits people can be ritch but anyway ...
#[derive(PartialEq,Debug)]
pub struct TL(f32);  // The poorest & the cheapest one ...

pub trait ToUSDv<F> 
{
    fn to_uv(&self,F) -> f32; //where F is any type
}

pub trait FromUSDv<F>
{
    fn from_uv(&self,f32)->F;
}

pub struct Ex{
    tl:f32,
    usd:f32,
}

impl ToUSDv<TL> for Ex{
    fn to_uv(&self,g:GBP) ->f32{
        (g.0 as f32) * self.tl
    }
}

impl FromUSDv<TL> for Ex{
    fn from_uv(&self,f:f32)->TL
    {
        
    }
}

pub fn test_it()
{

}