use dspcalc::dsp::{
    item::{
        Cargo, Resource,
        ResourceType::{self},
    },
    mine::mines,
    recipe::{flatten_recipes, print_recipe, proliferator_recipes},
};
use dspdb::{item::items, recipe};

fn main() {
    let need_white_cube = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 6006,
            level: 0,
        }),
        num: 10000.0,
    };

    let need_proliferator_mk3 = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 1143,
            level: 0,
        }),
        num: 10000.0,
    };

    let raw_recipes = recipe::recipes();
    let raw_items = items();

    // FIXME 光子不是原矿
    // TODO 接入禁用公式列表（直接移除对应的约束）
    // TODO 增加真正的原矿化（直接移除相关的公式）
    // TODO 物流卡顿：爪子进出建筑，大塔，传送带等

    let mines = mines(&raw_items);
    // dbg!(mines);

    // 展平所有基础公式
    let flatten_basic_recipes = flatten_recipes(&raw_recipes.data_array);
    // 所有的喷涂公式
    let proliferator_recipes = proliferator_recipes(&raw_items.data_array);

    // 找出所有在公式中出现过的资源
    let all_recipes = [flatten_basic_recipes, proliferator_recipes, mines].concat();
    // for recipe in &all_recipes {
    //     print_recipe(60.0, recipe, &raw_items.data_array);
    // }

    let needs = vec![need_white_cube];
    // let needs = vec![need_proliferator_mk3];

    // FIXME 消除这个unwarp
    let solutions = dspcalc::solver::solve(&all_recipes, &needs).unwrap();
    let price = solutions.iter().map(|a| a.num).sum::<f64>();
    for solution in solutions {
        print_recipe(solution.num, &solution.recipe, &raw_items.data_array);
    }
    print!("总成本：{}", price / 3600.0);
}

// TODO 群友建议
// 1、原矿化列表显示数量
// 2、生产策略/需求列表保存后，重新加载尚未包含原来“视为原矿的部分”
// 3、
//      “添加现有产线”尚未根据“批量预设”的增产和机器类型显示，还需要重新选择增产剂、增产模式、工厂类型。
//      “添加现有产线”默认工厂数量建议为1，现为10。
//      “添加现有产线”新增的配方和原配方位于同一位置，现在为置顶，
// 4、建筑统计可以不显示建筑名称，鼠标移上去再显示，原矿化列表现在就是移上去显示名称
// 5、支持导出分享策略（跨设备、跨用户共享）
// 6、计算结果导出excel，那么下面这些都简单了
// 7、支持查看多用途物品各配方的占比，例如白糖产线中电路板多少用于蓝糖、多少用于处理器。可以像现在的多来源 氢 那样展示。
// 8、显示单配方根据成品和原料分别需要的传送带（设置中设定传送带堆叠数量）数量
// 9、支持配方勾选、标注、改色等区别于其他配方的操作
// 10、支持方案对比，将两个保存的生产策略/需求列表根据原矿化列表、建筑统计、电力等信息进行对比
// 11、添加需求数量时根据机器数量选择，例如10个熔炉产出的钛合金
