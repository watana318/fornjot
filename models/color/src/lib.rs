use fj::core::{
    objects::Solid,
    operations::{
        presentation::SetColor,
        split::SplitFace,
        update::{UpdateFace, UpdateShell, UpdateSolid},
    },
};

pub fn model(core: &mut fj::core::Instance) -> Solid {
    let size = 1.;
    let cuboid = cuboid::model([size, size, size], core);

    cuboid.update_shell(
        cuboid.shells().only(),
        |shell, core| {
            let shell = shell.update_face(
                shell.faces().first(),
                |face, core| {
                    [face.update_region(
                        |region, _| region.set_color([0., 1., 0.]),
                        core,
                    )]
                },
                core,
            );

            // Split colored face, to make sure the same color is applied to the
            // two derived faces.
            let shell = {
                let face = shell.faces().first();
                let line = {
                    let cycle = face.region().exterior();

                    [
                        (cycle.half_edges().nth(0).unwrap(), [0.2]),
                        (cycle.half_edges().nth(2).unwrap(), [0.2]),
                    ]
                };

                let (shell, _) = shell.split_face(face, line, core);
                shell
            };

            [shell]
        },
        core,
    )
}
