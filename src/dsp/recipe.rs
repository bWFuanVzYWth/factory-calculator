use crate::dsp::item::Cargo;
use dspdb::{item::ItemData, recipe::RecipeItem};

use super::{
    building::BuildingType,
    item::{Resource, ResourceType::Direct},
    proliferator::Proliferator,
};

#[derive(Clone, Debug)]
pub struct RecipeFmtInfo {
    pub name: String, // 公式的名字
    pub level: usize, // 使用的增产剂
    pub speed_up: bool,
    pub building_type: BuildingType, // 生产于什么建筑
}

impl Default for RecipeFmtInfo {
    fn default() -> Self {
        Self {
            name: "Unknown Building".to_string(),
            level: 0,
            speed_up: true,
            building_type: BuildingType::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,   // 原料
    pub results: Vec<Resource>, // 产物
    pub time: f64,              // 公式耗时，单位帧
    pub info: RecipeFmtInfo,    // 不参与计算的信息
}

impl Recipe {
    fn create_recipe(
        recipe_item: &RecipeItem,
        level: usize,
        modify_result_num: impl Fn(f64) -> f64,
        modify_time: impl Fn(f64) -> f64,
        info: RecipeFmtInfo,
    ) -> Self {
        Self {
            items: recipe_item
                .items
                .iter()
                .zip(recipe_item.item_counts.iter())
                .map(|(item, count)| Resource {
                    resource_type: Direct(Cargo {
                        item_id: *item,
                        level,
                    }),
                    num: *count as f64,
                })
                .collect(),
            results: recipe_item
                .results
                .iter()
                .zip(recipe_item.result_counts.iter())
                .map(|(res, count)| Resource {
                    resource_type: Direct(Cargo {
                        item_id: *res,
                        level: 0,
                    }),
                    num: modify_result_num(*count as f64),
                })
                .collect(),
            time: modify_time(recipe_item.time_spend as f64)
                * BuildingType::from_recipe_item(recipe_item).time_scale(),
            info,
        }
    }

    fn accelerate(recipe_item: &RecipeItem, level: usize) -> Self {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            level,
            speed_up: true,
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        Self::create_recipe(
            recipe_item,
            level,
            |num| num,
            |time| time / Proliferator::accelerate(level),
            info,
        )
    }

    fn productive(recipe_item: &RecipeItem, level: usize) -> Self {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            level,
            speed_up: false,
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        Self::create_recipe(
            recipe_item,
            level,
            |num| num * Proliferator::increase(level),
            |time| time,
            info,
        )
    }

    fn recipes_accelerate(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        for level in 1..=Proliferator::MAX_INC_LEVEL {
            recipes.push(Self::accelerate(recipe_item, level));
        }
    }

    fn recipes_productive(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        if !recipe_item.non_productive {
            for level in 1..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self::productive(recipe_item, level));
            }
        }
    }

    fn recipe_vanilla(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            level: 0,
            speed_up: false,
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        recipes.push(Self::create_recipe(
            recipe_item,
            0,
            |num| num,
            |time| time,
            info,
        ));
    }

    #[must_use]
    pub fn flatten_recipes(basic_recipes: &[RecipeItem]) -> Vec<Self> {
        let mut recipes = Vec::new();
        for recipe_item in basic_recipes {
            Self::recipe_vanilla(&mut recipes, recipe_item);
            Self::recipes_productive(&mut recipes, recipe_item);
            Self::recipes_accelerate(&mut recipes, recipe_item);
        }
        recipes
    }

    fn generate_proliferator_recipe(
        recipes: &mut Vec<Self>,
        item_data: &ItemData,
        proliferator: &Proliferator,
    ) {
        const STACK: f64 = 4.0;
        const PROLIFERATOR_TIME: f64 = 2.0;
        for cargo_level in 1..=Proliferator::inc_level(proliferator) {
            for proliferator_level in 0..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self {
                    items: vec![
                        Resource::from_item_level(item_data.id, 0, STACK),
                        Resource::from_item_level(
                            Proliferator::item_id(proliferator),
                            proliferator_level,
                            ((Proliferator::inc_level(proliferator) as f64) / (cargo_level as f64))
                                * STACK
                                / (Proliferator::life(proliferator, proliferator_level) as f64),
                        ),
                    ],
                    results: vec![Resource::from_item_level(item_data.id, cargo_level, STACK)],
                    time: PROLIFERATOR_TIME,
                    info: RecipeFmtInfo {
                        name: "喷涂".to_string(),
                        building_type: BuildingType::喷涂机,
                        ..RecipeFmtInfo::default()
                    },
                });
            }
        }
    }
    // TODO 耗电量

    #[must_use]
    pub fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Self> {
        let mut recipes = Vec::new();
        for item_data in items_data {
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK3);
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK2);
            Self::generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK1);
        }
        recipes
    }

    #[must_use]
    pub fn mines(raw_items: &dspdb::item::ItemProtoSet) -> Vec<Recipe> {
        let mut mines = Vec::new();
        for item in &raw_items.data_array {
            let is_mine = |item: &ItemData| !item.mining_from.is_empty();
            if is_mine(item) {
                let tmp = Recipe {
                    items: Vec::new(),
                    results: vec![Resource {
                        resource_type: Direct(Cargo {
                            item_id: item.id,
                            level: 0,
                        }),
                        num: 4.0, // TODO 根据采矿等级设置成本，或者增加原矿化标记字段，不计成本
                    }],
                    time: 1.0,
                    info: RecipeFmtInfo {
                        name: "采矿".to_string(),
                        building_type: BuildingType::矿机,
                        ..RecipeFmtInfo::default()
                    },
                };
                mines.push(tmp);
            }
        }
        mines
    }
}
