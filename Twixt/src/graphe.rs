use std::usize;
pub struct Graphe {
    wall: Vec<((usize, usize), (usize, usize))>, //on recupère la case de la tour dans le tableux d'une dimension
}

impl Graphe {
    pub fn new() -> Self {
        Self { wall: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.wall = Vec::new();
    }

    // pub fn check_win(&self) -> bool {
    //     true
    // } //fonction de check

    pub fn remove_wall_connect_to(&mut self, tower: &(usize, usize)) {
        for (tower1,tower2) in self.wall.clone().iter() {
            if tower1 == tower || tower2 == tower {
                self.remove_wall(tower1, tower2);
            }
        }
    }

    pub fn add_wall(&mut self, tower1: &(usize, usize), tower2: &(usize, usize)) {
        //ajoute un mur dans la liste
        //si en placent la tour avec la plus petite indice en premier
        if tower1 > tower2 {
            self.wall.push((tower2.clone(), tower1.clone()));
        } else {
            self.wall.push((tower1.clone(), tower2.clone()));
        }
    }

    pub fn liste_of_wall(&self) -> &Vec<((usize, usize), (usize, usize))> {
        &self.wall
    }

    pub fn remove_wall(&mut self, tower1: &(usize, usize), tower2: &(usize, usize)) {
        if tower1 > tower2 {
            self.wall.retain(|&x| x != (tower2.clone(), tower1.clone()));
        } else {
            self.wall.retain(|&x| x != (tower1.clone(), tower2.clone()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_wall() {
        let mut graphe: Graphe = Graphe::new();
        let t1: (usize, usize) = (0, 0);
        let t2: (usize, usize) = (1, 0);
        let t3: (usize, usize) = (0, 1);
        graphe.add_wall(&t1, &t2);
        graphe.add_wall(&t1, &t3);
        graphe.add_wall(&t2, &t3);
        graphe.remove_wall(&t1, &t2);
        graphe.remove_wall(&t1, &t3);
        assert_eq!(1, graphe.liste_of_wall().len())
    }

    #[test]
    fn test_add_wall() {
        let mut graphe: Graphe = Graphe::new();
        let t1: (usize, usize) = (0, 0);
        let t2: (usize, usize) = (1, 0);
        let t3: (usize, usize) = (0, 1);
        graphe.add_wall(&t1, &t2);
        graphe.add_wall(&t1, &t3);
        graphe.add_wall(&t2, &t3);
        assert_eq!(3, graphe.liste_of_wall().len())
    }

    #[test]
    fn test_remove_wall_connect_to() {
        let mut graphe: Graphe = Graphe::new();
        let t1: (usize, usize) = (0, 0);
        let t2: (usize, usize) = (1, 0);
        let t3: (usize, usize) = (0, 1);
        graphe.add_wall(&t1, &t2);
        graphe.add_wall(&t1, &t3);
        graphe.add_wall(&t2, &t3);
        graphe.remove_wall_connect_to(&t1);
        assert_eq!(1, graphe.liste_of_wall().len())
    }
}
