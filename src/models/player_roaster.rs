use anyhow::Result;



#[derive(Debug, Clone)]
pub struct PlayerRoaster {
    pub player_list: Vec<Player>,
}

impl PlayerRoaster {
    pub fn new() -> Self {
        Self {
            player_list: vec![
                Player {
                    name: "Paul McBeth",
                    strength: 70,
                },
                Player {
                    name: "Richard Wysocki",
                    strength: 43,
                },
                Player {
                    name: "Calvin Heimburg",
                    strength: 65,
                },
                Player {
                    name: "Eagle Wynne McMahon",
                    strength: 89,
                },
                Player {
                    name: "Chris Dickerson",
                    strength: 37,
                }
            ],
        }
    }

    pub fn streng_by_index(&self, index: usize) -> Option<i32> {
        match self.player_list.get(index) {
            Some(player) => Some(player.strength),
            None => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub name: &'static str,
    pub strength: i32,
}

