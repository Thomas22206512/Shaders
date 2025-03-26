use graphe::Graphe;
use macroquad::prelude::*;
use tablier::Tablier;

mod graphe;
mod tablier;

enum Sortie {
    Rien,
    ChangeJou,
    Sortie,
}

fn is_secant(
    a: &(usize, usize),
    b: &(usize, usize),
    c: &(usize, usize),
    d: &(usize, usize),
) -> bool {
    let a = (a.0 as f32, a.1 as f32);
    let b = (b.0 as f32, b.1 as f32);
    let c = (c.0 as f32, c.1 as f32);
    let d = (d.0 as f32, d.1 as f32);
    let alpha: f32 = (d.0 - c.0) * (c.1 - a.1) - (d.1 - c.1) * (c.0 - a.0);
    let beta: f32 = (d.0 - c.0) * (b.1 - a.1) - (d.1 - c.1) * (b.0 - a.0);
    let gamma: f32 = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
    let zeta = alpha / beta;
    let omega = gamma / beta;
    0. < zeta && zeta < 1. && 0. < omega && omega < 1.
}

fn draw_background() {
    clear_background(BLACK);
}//Met le fond en noir

fn real_position(
    tablier: &Tablier,
    offset_x: &f32,
    offset_y: &f32,
    point: &(usize, usize),
) -> (f32, f32) {
    let squares = tablier.get_size(); // le nombre de carrées
    let sq_size = (screen_height() - offset_y * 2.) / squares as f32;
    let (i, j) = point.clone();
    let x = offset_x + sq_size * (i + 1) as f32 - (sq_size / 2.);
    let y = offset_y + sq_size * (j + 1) as f32 - (sq_size / 2.);
    (x, y)
}

fn is_not_corner(i: usize, j: usize, tablier: &Tablier) -> bool {
    //enleve les coins
    !((i == 0 || i == tablier.get_size() - 1) && (j == 0 || j == tablier.get_size() - 1))
}//Détecte si la position (i,j) n'est pas un coin

fn is_in_box(mouse_pos: &Vec2, x1: &f32, y1: &f32, x2: &f32, y2: &f32) -> bool {
    (mouse_pos.x >= *x1) && (mouse_pos.x < *x2) && (mouse_pos.y >= *y1) && (mouse_pos.y < *y2)
} //Détecte si la souris est bien situer dans le rectangle qui est défini par deux point (x1,y1) et (x2,y2)

fn draw(
    tablier: &Tablier,
    graphe1: &Graphe,
    graphe2: &Graphe,
    game_size: &f32,
    offset_x: &f32,
    offset_y: &f32,
    game_mode: &u8,
    joueur: &u8,
    double_click: &[(usize, usize)],
) {
    draw_background(); //Fond Noir
    match game_mode {
        0 => {
            //Partie Ecrant de sélection
            let text = vec!["1 VS 1","Algo min-max","IA","Credit"];
            let y_pos = vec![-15., 30., 65., 105.];
            let mouse_pos = Vec2::from(mouse_position());
            for i in 0..=3 {
                let text_size = measure_text(text[i], None, 30. as _, 1.0);
                draw_rectangle((screen_width() / 2. - text_size.width / 2.) - 1., (screen_height() / 2. - text_size.height / 2.) + y_pos[i] + 4., text_size.width + 10., -text_size.height - 10., LIGHTGRAY);
                match is_in_box(&mouse_pos, &((screen_width() / 2. - text_size.width / 2.) - 5.), &((screen_height() / 2. - text_size.height / 2.) + y_pos[i] -text_size.height - 10.), &((screen_width() / 2. - text_size.width / 2.) - 5. + text_size.width + 10.), &((screen_height() / 2. - text_size.height / 2.) + y_pos[i])) {
                    true => draw_rectangle((screen_width() / 2. - text_size.width / 2.) - 5., (screen_height() / 2. - text_size.height / 2.) + y_pos[i], text_size.width + 10., -text_size.height - 10., LIGHTGRAY),
                    false => draw_rectangle((screen_width() / 2. - text_size.width / 2.) - 5., (screen_height() / 2. - text_size.height / 2.) + y_pos[i], text_size.width + 10., -text_size.height - 10., WHITE)
                }
                draw_text(
                    text[i],
                    screen_width() / 2. - text_size.width / 2.,
                    (screen_height() / 2. - text_size.height / 2.) - 20. + (i*40) as f32,
                    30.,
                    DARKGRAY,
                );
            }//Affiche les boutons
            let title = "TWIXT";
            let text_size = measure_text(title, None, 120. as _, 1.0);
            let color = vec![LIGHTGRAY, WHITE];
            for i in 0..2{
                draw_text(
                    title,
                    (screen_width() / 2.) - (text_size.width / 2.) + 3. -(i*3) as f32,
                    (screen_height() / 2. - text_size.height / 2.) - 77. -(i*3) as f32,
                    120.,
                    color[i],
                );
            }//Affiche le titre du jeux
        }
        1 => {
            //Partie Jeux
            let squares = tablier.get_size(); // le nombre de carrées
            let sq_size = (screen_height() - offset_y * 2.) / squares as f32; // la taille d'un carrée
            match joueur {
                0 => draw_rectangle(
                    (screen_width() - *game_size) / 2. + 5.,
                    (screen_height() - *game_size) / 2. + 5.,
                    *game_size - 10.,
                    game_size - 10.,
                    BLUE,
                ),
                1 => draw_rectangle(
                    (screen_width() - *game_size) / 2. + 5.,
                    (screen_height() - *game_size) / 2. + 5.,
                    *game_size - 10.,
                    game_size - 10.,
                    RED,
                ),
                _ => todo!(),
            }//Change la couleur de la marge selon qui à la main
            draw_rectangle(
                *offset_x,
                *offset_y,
                *game_size - 20.,
                game_size - 20.,
                WHITE,
            );//Fond du tablier
            for i in 1..squares {
                draw_line(
                    *offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
                draw_line(
                    offset_x + sq_size * i as f32,
                    *offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }//Affichage du damier (tablier)
            for i in 0..2 {
                draw_line(
                    offset_x + sq_size + sq_size * ((squares - 2) * i) as f32,
                    *offset_y,
                    offset_x + sq_size + sq_size * ((squares - 2) * i) as f32,
                    screen_height() - offset_y,
                    2.,
                    RED,
                );
                draw_line(
                    *offset_x,
                    offset_y + sq_size + sq_size * ((squares - 2) * i) as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size + sq_size * ((squares - 2) * i) as f32,
                    2.,
                    BLUE,
                );
            } //les rivières blue et rouge
            if double_click.len() == 1 {
                let (x, y) = double_click[0];
                draw_circle(
                    offset_x + sq_size * (x + 1) as f32 - (sq_size / 2.),
                    offset_y + sq_size * (y + 1) as f32 - (sq_size / 2.),
                    1. * (game_size / 100.),
                    GREEN,
                );
                let neighbors = tablier.get_neighbors(x, y);
                for (tx,ty) in neighbors {
                    draw_circle(
                        offset_x + sq_size * (tx + 1) as f32 - (sq_size / 2.),
                        offset_y + sq_size * (ty + 1) as f32 - (sq_size / 2.),
                        1. * (game_size / 100.),
                        LIME,
                    );
                }
            }//Affichage du dernier click et des voisins (Surbrillance)
            for i in 0..squares {
                for j in 0..squares {
                    if is_not_corner(i, j, tablier) {
                        match tablier.get_value(i, j) {
                            0 => draw_circle(
                                offset_x + sq_size * (i + 1) as f32 - (sq_size / 2.),
                                offset_y + sq_size * (j + 1) as f32 - (sq_size / 2.),
                                1. * (game_size / 200.),
                                BLACK,
                            ),
                            1 => draw_circle(
                                offset_x + sq_size * (i + 1) as f32 - (sq_size / 2.),
                                offset_y + sq_size * (j + 1) as f32 - (sq_size / 2.),
                                1.5 * (game_size / 200.),
                                BLUE,
                            ),
                            2 => draw_circle(
                                offset_x + sq_size * (i + 1) as f32 - (sq_size / 2.),
                                offset_y + sq_size * (j + 1) as f32 - (sq_size / 2.),
                                1.5 * (game_size / 200.),
                                RED,
                            ),
                            _ => todo!(),
                        }
                    }
                }
            }//Affichage des tours vide(s), rouge ou bleu
            for (t1, t2) in graphe1.liste_of_wall() {
                let (x1, y1) = real_position(tablier, offset_x, offset_y, t1);
                let (x2, y2) = real_position(tablier, offset_x, offset_y, t2);
                draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 2.0, BLUE);
            } //Affichage des murs Bleu
            for (t1, t2) in graphe2.liste_of_wall() {
                let (x1, y1) = real_position(tablier, offset_x, offset_y, t1);
                let (x2, y2) = real_position(tablier, offset_x, offset_y, t2);
                draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 2.0, RED);
            } //Affichage des murs Rouge
        }
        _ => todo!(),
    }
} //fonction d'affichage, on utilise des Arrays à la place des Vec ici car on n'effectue pas de modification sur la taille de la liste mais juste une lecture.

fn update_graphe(
    tablier: &mut Tablier,
    graphe1: &mut Graphe,
    graphe2: &mut Graphe,
    t1: (usize, usize),
    t2: (usize, usize),
) {
    for tower in tablier.get_neighbors(t1.0, t1.1) {
        if tower == t2 {
            let mut no_secant: bool = true;
            let walls = vec![graphe1.liste_of_wall(), graphe2.liste_of_wall()]; //besoin de verif pour le graphe1 et graphe2
            for graphe in walls {
                //donc iter sur les deux
                for (t3, t4) in graphe {
                    if is_secant(&t1, &t2, t3, t4) {
                        //si un seul de tous les walls sont sécantes alors on ne créé pas de nouveau mur
                        no_secant = false;
                    }
                }
            }
            if no_secant {
                graphe1.add_wall(&t1, &t2); //cas ou le nouveau mur ne posse pas de problème
            }
        }
    }
}//Fonction qui met a jour le premier graphe passer en entrer si il peut crée un nouveau mur

fn handle_input(
    tablier: &mut Tablier,
    graphe1: &mut Graphe,
    graphe2: &mut Graphe,
    double_click: &mut Vec<(usize, usize)>,
    offset_x: &f32,
    offset_y: &f32,
    can_place: &mut bool,
    joueur: u8,
    game_mode: &mut u8
) -> Sortie {
    if is_key_pressed(KeyCode::Escape) {
        return Sortie::Sortie; 
    }// Stoppe le programme
    let mouse_pos = Vec2::from(mouse_position());
    let i = ((mouse_pos.x - *offset_x)
        / ((screen_width() - offset_x * 2.) / tablier.get_size() as f32)) as usize; // coordonnée x dans le tablier
    let j = ((mouse_pos.y - *offset_y)
        / ((screen_height() - offset_y * 2.) / tablier.get_size() as f32)) as usize; // coordonnée y dans le tablier
    if is_mouse_button_pressed(MouseButton::Left) {
        if *game_mode > 0 {//Partie de jeux
            if is_in_box(
                &mouse_pos,
                offset_x,
                offset_y,
                &(screen_width() - offset_x),
                &(screen_height() - offset_y),
            )
            //Vérifie que le click est bien fait au niveau du tablier car sinon donne des résultat en dehors de l' array
            {
                if is_not_corner(i, j, tablier) { //Vérifie que ce n'est pas un des quatres coins
                    if tablier.get_value(i, j) == 0 && *can_place {//Vérifie que la case soit libre et qu'il n'est pas déjà posé une tour
                        match joueur {
                            0 => {
                                if i > 0 && i < tablier.get_size() - 1 {//Vérifie qu'il ne soit pas au niveau du bord ennemis
                                    tablier.set_value(i, j, 1);
                                } else {
                                    return Sortie::Rien;//Sinon sort pour éviter de modifier can_place
                                }
                            }
                            1 => {
                                if j > 0 && j < tablier.get_size() - 1 {//Vérifie qu'il ne soit pas au niveau du bord ennemis
                                    tablier.set_value(i, j, 2);
                                } else {
                                    return Sortie::Rien;//Sinon sort pour éviter de modifier can_place
                                }
                            }
                            _ => todo!(),
                        }
                        *can_place = false;//Enlève la possibilité au joueur de reposer une tour
                    }
                    if tablier.get_value(i, j) == joueur + 1 && !*can_place{//si le click est fait sur une tour lui appartenant
                        if double_click.len() == 0 { //Si c'est le premier click alors 
                            double_click.push((i, j));//stocke la position du click
                        } else {
                            let (x, y) = double_click[0];
                            match joueur {
                                0 => update_graphe(tablier, graphe1, graphe2, (x, y), (i, j)),
                                1 => update_graphe(tablier, graphe2, graphe1, (x, y), (i, j)),
                                _ => todo!(),
                            }
                            double_click.remove(0);
                        }//Essaye de crée un mur si possible
                    }
                }
            } //vérifie si le click a bien été effectuer sur le tablier
        } else { //Partie Ecran de séléction
            let text = vec!["1 VS 1","Algo min-max","IA","Credit"];
            let y_pos = vec![-15., 30., 65., 105.];
            for i in 0..=3{
                let text_size = measure_text(text[i], None, 30. as _, 1.0);
                if is_in_box(&mouse_pos,&((screen_width() / 2. - text_size.width / 2.) - 5.), &( (screen_height() / 2. - text_size.height / 2.) + y_pos[i] -text_size.height - 10.) , &( (screen_width() / 2. - text_size.width / 2.) +text_size.width + 5.), &((screen_height() / 2. - text_size.height / 2.) + y_pos[i])) {
                    *game_mode = 1; //(i + 1) as u8 a terme mais pour l'instant 1
                }//Vérifie si le click à été fait sur un des 4 boutons
            }
        }
    }
    if is_mouse_button_pressed(MouseButton::Right) && *can_place && *game_mode > 0 {//Partie Suppression
        if is_not_corner(i, j, tablier) {//Si le click est n'est pas sur un des 4 coins
            if tablier.get_value(i, j) == joueur + 1 {//Si la tour lui appartient
                if double_click.len() == 0 { //Si c'est le premier click on stocke la position
                    double_click.push((i, j));
                } else {
                    let (x, y) = double_click[0];
                    if (x,y) == (i,j) {//Cas où on supprime une tour
                        tablier.set_value(i, j, 0);//libère la case
                        match joueur {
                            0 => graphe1.remove_wall_connect_to(&(i,j)),
                            1 => graphe2.remove_wall_connect_to(&(i,j)),
                            _ => todo!()
                        }//Supprime la tours et les arrètes qui y sont relier
                    } else {//cas ou on supprime un mur
                        match joueur {
                            0 => graphe1.remove_wall(&(x, y), &(i, j)),
                            1 => graphe2.remove_wall(&(x, y), &(i, j)),
                            _ => todo!(),
                        }
                    }
                    double_click.remove(0);//supprime la position une fois utilisé
                }
            }
        }
    }
    if is_mouse_button_pressed(MouseButton::Middle) || is_key_pressed(KeyCode::Space) && *game_mode > 0{
        if *can_place == false {
            return Sortie::ChangeJou;
        }
    }//Change la main (joueur)
    Sortie::Rien
} //fonction de gestion des inputs

fn update_model(_tablier: &mut Tablier, _graphe1: &mut Graphe, _graphe2: &mut Graphe) {} //fonction du gère les mouvement si besoin genre des animations d'effet ect..

#[macroquad::main("Twixt")]
async fn main() {
    let mut tablier: Tablier = Tablier::new(24);
    // tablier.set_value(1, 1, 1); //test couleur
    let mut joueur: u8 = 0;
    let mut graphe1: Graphe = Graphe::new();
    let mut graphe2: Graphe = Graphe::new();
    let mut game_mode: u8 = 0; //défini le mode de jeux  0 => écran d'acceuil, 1 => 1VS1 ect...
    let mut double_click: Vec<(usize, usize)> = Vec::new();
    let mut can_place: bool = true;
    loop {
        let game_size = screen_width().min(screen_height()); // la taille de la fenêtre
        let offset_x = (screen_width() - game_size) / 2. + 10.; // marge en x
        let offset_y = (screen_height() - game_size) / 2. + 10.; // marge en y
        //les trois variable d'avant sont pour que le jeux gère automatiquement la taille de l'écran et pas besoin qu'il soit mutable car recalculer à chaque itération de la loop

        draw(
            &tablier,
            &graphe1,
            &graphe2,
            &game_size,
            &offset_x,
            &offset_y,
            &game_mode,
            &joueur,
            &double_click,
        );

        let val = handle_input(
            &mut tablier,
            &mut graphe1,
            &mut graphe2,
            &mut double_click,
            &offset_x,
            &offset_y,
            &mut can_place,
            joueur,
            &mut game_mode
        );

        match val {
            Sortie::Sortie => {
                if game_mode > 0 {
                    game_mode = 0;
                    tablier.reset();
                    graphe1.reset();
                    graphe2.reset();
                    double_click = Vec::new();
                    can_place = true;
                    //Retour au Hub (menu de sélection)
                } else {
                    break;
                }// Sortie du jeux
            }
            Sortie::ChangeJou => {
                joueur = (joueur + 1) % 2;
                can_place = true;
                if double_click.len() == 1 {
                    double_click.remove(0);
                }
            }
            Sortie::Rien => {}
        }

        update_model(&mut tablier, &mut graphe1, &mut graphe2);

        next_frame().await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_secant() {
        let a = (0, 0);
        let b = (1, 1);
        let c = (0, 1);
        let d = (1, 0);
        assert_eq!(is_secant(&a, &b, &c, &d), true)
    }
}