pub struct Tablier {
    size: usize,
    array: [u8; 576], //utilisation d'array car la taille tu tablier est fixe
}
/// On définit 576 comme taille du tablier donc 24 X 24 mais
/// on peut utiliser moins

impl Tablier {

    pub fn new(size: usize) -> Self {
        assert!(size < 25);
        Self {
            size, // la taille du tablier utiliser max = 24
            array: [0; 576],
        }
    }

    pub fn reset(&mut self) {
        self.array = [0; 576];
    }

    pub fn get_size(&self) -> usize {
        self.size
    } // assécceur de size

    pub fn get_position(&self, i: usize, j: usize) -> usize {
        assert!(i < self.size);
        assert!(j < self.size);
        i % self.size + j * self.size
    } // qui prend en compte la taille du tablier utiliser self.size (reste a faire)

    pub fn get_value(&self, i: usize, j: usize) -> u8 {
        let pos = self.get_position(i, j);
        self.array[pos]
    } // assécceur de la valeur en position pos

    pub fn set_value(&mut self, i: usize, j: usize, value: u8) {
        let pos = self.get_position(i, j);
        self.array[pos] = value;
    } // mutateur de la valeur en position pos

    pub fn get_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mouvements: [(i8, i8); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];
        let color = self.get_value(i, j); //valeur de la case
        let mut rep: Vec<(usize, usize)> = Vec::with_capacity(8); //vecteur contenant les voisins
        let i: i8 = i as i8;
        let j: i8 = j as i8;
        let max: i8 = self.get_size() as i8;
        for (x, y) in mouvements {
            let dx = i + x;
            let dy = j + y;
            if 0 <= dx && dx < max && 0 <= dy && dy < max {
                if self.get_value(dx as usize, dy as usize) == color {
                    rep.push((dx as usize, dy as usize));
                }
            }
        }
        rep
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_position() {
        let tablier: Tablier = Tablier::new(3);
        assert_eq!(0, tablier.get_position(0, 0));
        assert_eq!(4, tablier.get_position(1, 1));
        assert_eq!(8, tablier.get_position(2, 2));
    }

    #[test]
    fn test_get_value() {
        let mut tablier: Tablier = Tablier::new(3);
        tablier.array[4] = 4;
        assert_eq!(4, tablier.get_value(1, 1));
    }

    #[test]
    fn test_set_value() {
        let mut tablier: Tablier = Tablier::new(3);
        tablier.set_value(1, 1, 4);
        assert_eq!(4, tablier.array[4]);
    }
    #[test]
    fn test_neighbors() {
        let mut tablier: Tablier = Tablier::new(5);
        tablier.set_value(2, 2, 1);
        tablier.set_value(0, 1, 1);
        tablier.set_value(3, 4, 1);
        assert_eq!(2, tablier.get_neighbors(2, 2).len())
    }
}
