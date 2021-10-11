use crate::sim::Material;

pub(crate) fn start_color(material: &Material) -> [u8; 3] {
    match material {
        Material::Sand => [224, 227, 129],
        Material::Water => todo!(),
        Material::Wall => todo!(),
        Material::Clone => todo!(),
    }
}
