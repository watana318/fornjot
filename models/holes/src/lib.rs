use fj::core::{
    objects::Solid,
    operations::{holes::AddHole, insert::Insert, update::UpdateSolid},
    services::Services,
    storage::Handle,
};

pub fn model(services: &mut Services) -> Handle<Solid> {
    let cuboid = cuboid::model(1., 1., 1., services);

    let hole_radius = 0.25;

    cuboid
        .update_shell(cuboid.shells().first(), |shell| {
            let bottom_face = shell.faces().first();

            let hole_position = [0., 0.];
            let hole_path = [0., 0., 0.5];

            shell
                .add_blind_hole(
                    bottom_face,
                    hole_position,
                    hole_radius,
                    hole_path,
                    services,
                )
                .insert(services)
        })
        .insert(services)
}
