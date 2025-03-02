use std::collections::{HashMap, HashSet};

use crate::data::dsp::{
    item::{Cargo, IndirectResource, Item, ResourceType},
    recipe::{recipes, BasicRecipe, Recipe, BASIC_RECIPES},
};
use good_lp::{
    clarabel, solvers::clarabel::ClarabelProblem, variable, variables, Expression, Solution,
    SolverModel, Variable,
};
use strum::IntoEnumIterator;

// 对每一种物品，生产必须大于需求
fn constraint_recipe(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    recipe_vars: &HashMap<usize, Variable>,
) {
    all_productions.iter().for_each(|production| {
        // 对每种产品
        // 找出所有生产它的公式，分别计算单位时间产能
        let all_find_production = recipes.iter().enumerate().filter(|(_, recipe)| {
            recipe
                .products
                .iter()
                .any(|resource| resource.resource_type == *production)
        });

        // 找出所有需求它的公式，分别计算单位时间需求
        let all_find_resource = recipes.iter().enumerate().filter(|(_, recipe)| {
            recipe
                .resources
                .iter()
                .any(|resource| resource.resource_type == *production)
        });

        let c_production = all_find_production
            .map(|(i, recipe)| {
                let var = recipe_vars.get(&i).unwrap();
                *var / time(recipe)
            })
            .sum::<Expression>();

        let c_resource = all_find_resource
            .map(|(i, recipe)| {
                let var = recipe_vars.get(&i).unwrap();
                *var / time(recipe)
            })
            .sum::<Expression>();

        problem.add_constraint(c_production.geq(c_resource));
    });
}

// 最小化建筑总数量
fn objective(recipe_vars: &HashMap<usize, Variable>) -> Expression {
    recipe_vars
        .iter()
        .map(|(_, variable)| variable)
        .sum::<Expression>()
}

fn find_all_production(recipes: &[Recipe]) -> HashSet<ResourceType> {
    let mut resources_type = HashSet::new();
    recipes.iter().for_each(|recipe| {
        recipe.products.iter().for_each(|product| {
            resources_type.insert(product.resource_type.clone());
        });
    });
    resources_type
}

fn find_all_resources(recipes: &[Recipe]) -> HashSet<ResourceType> {
    let mut resources_type = HashSet::new();
    recipes.iter().for_each(|recipe| {
        recipe.resources.iter().for_each(|resource| {
            resources_type.insert(resource.resource_type.clone());
        });
    });
    resources_type
}

fn time(recipe: &Recipe) -> f64 {
    recipe
        .resources
        .iter()
        .find(|resource| {
            if let ResourceType::Indirect(indirect) = &resource.resource_type {
                return indirect == &IndirectResource::Time;
            }
            false
        })
        .unwrap()
        .num
}

// TODO 传入需求和约束，返回求解过程和结果
pub fn solve() {
    // 生成所有公式
    let all_recipes = recipes(BASIC_RECIPES);
    // 找出所有在公式中出现过的资源
    let all_resources = find_all_resources(&all_recipes);
    let all_productions = find_all_production(&all_recipes);
    // dbg!(recipes);

    // 定义变量，每个变量代表一个公式的调用次数
    let mut recipe_vars = HashMap::new();
    let mut model = variables!();
    all_recipes.iter().enumerate().for_each(|(i, recipe)| {
        let var = model.add(variable().min(0.0));
        recipe_vars.insert(i, var); // recipes_index -> variables
    });

    // TODO 多种待优化目标，如最小化加权原矿，最小化占地
    let objective = objective(&recipe_vars);

    let mut problem = model.minimise(objective).using(clarabel);

    // TODO 根据公式表生成约束
    // 自动生成 constraint_recipe
    constraint_recipe(&mut problem, &all_recipes, &all_productions, &recipe_vars);

    let need_type = ResourceType::Direct(Cargo {
        item: Item::高能石墨,
        point: 0,
    });
    assert!(all_productions.contains(&need_type) == true); // 确保待求解的物品存在
    let need_num = 100.0;

    let constraint_need = problem.add_constraint(
        recipe_vars
            .iter()
            .map(|(recipes_index, variable)| {
                let need_resource = all_recipes[*recipes_index]
                    .products
                    .iter()
                    .find(|product| product.resource_type == need_type);

                let time = time(&all_recipes[*recipes_index]);

                match need_resource {
                    Some(product) => {
                        // 计算单位时间产能
                        (product.num / time) * (*variable)
                    }
                    None => 0.0.into(),
                }
            })
            .sum::<Expression>()
            .geq(need_num),
    );

    let solution = problem.solve().unwrap();

    all_recipes.iter().enumerate().for_each(|(i, recipe)| {
        let num = solution.value(*recipe_vars.get(&i).unwrap());
        if num > 0.00001 {
            println!("公式:{:#?}, 数量: {}", recipe, num);
        }
    });
}
