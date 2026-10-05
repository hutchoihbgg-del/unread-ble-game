use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct FollowCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, follow_player, jump_reset_hint))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Some obstacles / boxes to walk around
    for (x, z) in [(-5.0, -5.0), (5.0, -3.0), (0.0, 6.0), (-4.0, 4.0)] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.3, 0.2))),
            Transform::from_xyz(x, 0.75, z),
        ));
    }

    // Player cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 0.9))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player { speed: 6.0 },
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera (third person follow)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FollowCamera,
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    player_params: Query<&Player>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    let Ok(player) = player_params.single() else {
        return;
    };

    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if dir.length() > 0.0 {
        dir = dir.normalize();
        transform.translation += dir * player.speed * time.delta_secs();

        // Face movement direction
        let target_yaw = dir.x.atan2(-dir.z);
        let current = transform.rotation.to_euler(EulerRot::YXZ).0;
        let new_yaw = current + (target_yaw - current) * 10.0 * time.delta_secs();
        transform.rotation = Quat::from_rotation_y(new_yaw);

        // Clamp to ground area
        transform.translation.x = transform.translation.x.clamp(-24.0, 24.0);
        transform.translation.z = transform.translation.z.clamp(-24.0, 24.0);
    }

    // Simple jump with Space
    if keys.just_pressed(KeyCode::Space) && transform.translation.y <= 0.51 {
        transform.translation.y = 2.0;
    }
    // Gravity fall back
    if transform.translation.y > 0.5 {
        transform.translation.y -= 6.0 * time.delta_secs();
        if transform.translation.y < 0.5 {
            transform.translation.y = 0.5;
        }
    }
}

fn follow_player(
    player: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera: Query<&mut Transform, With<FollowCamera>>,
) {
    let Ok(p) = player.single() else {
        return;
    };
    let Ok(mut c) = camera.single_mut() else {
        return;
    };
    let offset = Vec3::new(0.0, 6.0, 10.0);
    let target = p.translation + offset;
    c.translation = c.translation.lerp(target, 5.0 * 0.016);
    c.look_at(p.translation + Vec3::Y, Vec3::Y);
}

fn jump_reset_hint() {
    // runs once hint via startup would spam; kept empty for simplicity
}
