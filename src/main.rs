struct Triangle {
    base: u32,
    side: u32,
}

impl Triangle {
    fn calculate_perimetre(&self) -> u32 {
        self.base + (2 * self.side)
    }

    fn is_base_greater_than_side(&self) -> bool {
        self.base > self.side
    }

    fn is_triangle_bigger(&self, other: &Triangle) -> bool {
        self.calculate_perimetre() > other.calculate_perimetre()
    }
}

fn main() {
    // Triangle isocèle: base + (2* côté)
    //x = base, y = c$oté
    let first_tirangle = Triangle {
        base: 15,
        side: 12
    };

    let second_tirangle = Triangle {
        base: 22,
        side: 25
    };

    println!(
        "Notre first_tirangle: Le périmètre d'un first_tirangle isocèle de base {}cm et de côté {}cm est égal à: {}cm ",
        first_tirangle.base, first_tirangle.side, first_tirangle.calculate_perimetre()
    );

    println!("La base est plus grande que les côtés: {}", first_tirangle.is_base_greater_than_side());
    println!("La base est plus grande que les côtés: {}", second_tirangle.is_base_greater_than_side());

    println!("Est ce que le 1er triangle est plus grand que le 2eme: {} (1er/2eme triangle{} > {})",
    first_tirangle.is_triangle_bigger(&second_tirangle), first_tirangle.calculate_perimetre(), second_tirangle.calculate_perimetre());
}