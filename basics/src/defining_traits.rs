#[derive(PartialEq,Debug)]
pub struct USD(f32); // we are not ritch so f32 will be enough
#[derive(PartialEq,Debug)]
pub struct GBP(f32); // Brits people can be ritch but anyway ...
#[derive(PartialEq,Debug)]
pub struct TL(f32);  // The poorest & the cheapest one ...

pub trait ToUSD{
    fn to_usd(&self)->USD;
}

pub trait ToString{
    fn to_string(&self)->String;
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD
    {
        USD(self.0 * 1.3)
    }
}
impl ToUSD for TL
{
    fn to_usd(&self) -> USD
    {
        USD(self.0 / 7.78)
    }
}

impl ToString for USD
{
    fn to_string(&self)->String
    {
        format!("{} $",self.0)
    }
}

impl ToString for TL
{
    fn to_string(&self)->String
    {
        format!("{} TL",self.0)
    }
}

pub fn test_it()
{
    let tl = TL(1.);
    println!("{} is {}",tl.to_string(),tl.to_usd().to_string());
}
