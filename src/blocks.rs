use crate::KarnaughDiagram;

#[derive(Debug, Default)]
pub(crate) struct BlocksFound {
    pub zeros: Vec<Rectangle>,
    pub ones: Vec<Rectangle>
}

#[derive(Debug, Default)]
pub struct Rectangle {
    pub elements: Vec<ElementCoordinate>,
    pub width: usize,
    pub height: usize
}

pub type ElementCoordinate = (usize, usize);

pub(crate) fn find_blocks(diagram: &KarnaughDiagram) -> BlocksFound {
    let mut blocks = BlocksFound::default();

    let mut visited = vec![vec![false; diagram[0].len()]; diagram.len()];

    for (y, row) in diagram.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if !visited[y][x] {
                let mut elements = vec![];
                dfs(diagram, &mut visited, value, y, x, &mut elements);
                let rectangle = Rectangle {
                    elements,
                    // Here you need to calculate width and height of the block
                    // It's non-trivial because of "looped" edges of the Karnaugh table
                    // So we leave it as placeholders
                    width: 0,
                    height: 0,
                };

                if value == 0 {
                    blocks.zeros.push(rectangle);
                } else {
                    blocks.ones.push(rectangle);
                }
            }
        }
    }

    blocks
}

fn dfs(diagram: &KarnaughDiagram, visited: &mut Vec<Vec<bool>>, value: i32, y: usize, x: usize, elements: &mut Vec<ElementCoordinate>) {
    if diagram[y][x] != value || visited[y][x] {
        return;
    }

    visited[y][x] = true;
    elements.push((y, x));

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let n = diagram.len();
    let m = diagram[0].len();

    for (dy, dx) in dir {
        let ny = ((y as i32 + dy + n as i32) % n as i32) as usize;
        let nx = ((x as i32 + dx + m as i32) % m as i32) as usize;
        dfs(diagram, visited, value, ny, nx, elements);
    }
}