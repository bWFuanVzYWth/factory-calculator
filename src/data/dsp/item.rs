use num::rational::Rational64;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

// TODO 英文的类型别名
#[derive(Debug, EnumIter, EnumString, EnumCount, Display, Clone, Eq, PartialEq, Hash)]
pub enum Item {
    地基 = 1131,
    传送带 = 2001,
    高速传送带 = 2002,
    极速传送带 = 2003,
    分拣器 = 2011,
    高速分拣器 = 2012,
    极速分拣器 = 2013,
    集装分拣器 = 2014,
    四向分流器 = 2020,
    自动集装机 = 2040,
    流速监测器 = 2030,
    喷涂机 = 2313,
    小型储物仓 = 2101,
    大型储物仓 = 2102,
    储液罐 = 2106,
    制造台mk1 = 2303,
    制造台mk2 = 2304,
    制造台mk3 = 2305,
    重组式制造台 = 2318,
    电力感应塔 = 2201,
    无线输电塔 = 2202,
    卫星配电站 = 2212,
    风力涡轮机 = 2203,
    火力发电厂 = 2204,
    微型聚变发电站 = 2211,
    地热发电站 = 2213,
    采矿机 = 2301,
    大型采矿机 = 2316,
    抽水站 = 2306,
    电弧熔炉 = 2302,
    位面熔炉 = 2315,
    负熵熔炉 = 2319,
    原油萃取站 = 2307,
    原油精炼厂 = 2308,
    化工厂 = 2309,
    分馏塔 = 2314,
    量子化工厂 = 2317,
    太阳能板 = 2205,
    蓄电器 = 2206,
    蓄电器_满 = 2207,
    电磁轨道弹射器 = 2311,
    射线接收站 = 2208,
    垂直发射井 = 2312,
    能量枢纽 = 2209,
    微型粒子对撞机 = 2310,
    人造恒星 = 2210,
    物流配送器 = 2107,
    行星内物流运输站 = 2103,
    星际物流运输站 = 2104,
    轨道采集器 = 2105,
    矩阵研究站 = 2901,
    自演化研究站 = 2902,
    高斯机枪塔 = 3001,
    高频激光塔 = 3002,
    聚爆加农炮 = 3003,
    磁化电浆炮 = 3004,
    导弹防御塔 = 3005,
    干扰塔 = 3006,
    信号塔 = 3007,
    行星护盾发生器 = 3008,
    战场分析基站 = 3009,
    近程电浆塔 = 3010,
    铁矿 = 1001,
    铜矿 = 1002,
    硅石 = 1003,
    钛石 = 1004,
    石矿 = 1005,
    煤矿 = 1006,
    木材 = 1030,
    植物燃料 = 1031,
    可燃冰 = 1011,
    金伯利矿石 = 1012,
    分形硅石 = 1013,
    光栅石 = 1014,
    刺笋结晶 = 1015,
    单极磁石 = 1016,
    铁块 = 1101,
    铜块 = 1104,
    高纯硅块 = 1105,
    钛块 = 1106,
    石材 = 1108,
    高能石墨 = 1109,
    钢材 = 1103,
    钛合金 = 1107,
    玻璃 = 1110,
    钛化玻璃 = 1119,
    棱镜 = 1111,
    金刚石 = 1112,
    晶格硅 = 1113,
    齿轮 = 1201,
    磁铁 = 1102,
    磁线圈 = 1202,
    电动机 = 1203,
    电磁涡轮 = 1204,
    超级磁场环 = 1205,
    粒子容器 = 1206,
    奇异物质 = 1127,
    电路板 = 1301,
    处理器 = 1303,
    量子芯片 = 1305,
    微晶元件 = 1302,
    位面过滤器 = 1304,
    粒子宽带 = 1402,
    电浆激发器 = 1401,
    光子合并器 = 1404,
    太阳帆 = 1501,
    水 = 1000,
    原油 = 1007,
    精炼油 = 1114,
    硫酸 = 1116,
    氢 = 1120,
    重氢 = 1121,
    反物质 = 1122,
    临界光子 = 1208,
    氢燃料棒 = 1801,
    氘核燃料棒 = 1802,
    反物质燃料棒 = 1803,
    奇异湮灭燃料棒 = 1804,
    塑料 = 1115,
    石墨烯 = 1123,
    碳纳米管 = 1124,
    有机晶体 = 1117,
    钛晶石 = 1118,
    卡西米尔晶体 = 1126,
    燃烧单元 = 1128,
    爆破单元 = 1129,
    晶石爆破单元 = 1130,
    引力透镜 = 1209,
    空间翘曲器 = 1210,
    湮灭约束球 = 1403,
    动力引擎 = 1407,
    推进器 = 1405,
    加力推进器 = 1406,
    配送运输机 = 5003,
    物流运输机 = 5001,
    星际物流运输船 = 5002,
    框架材料 = 1125,
    戴森球组件 = 1502,
    小型运载火箭 = 1503,
    增产剂mk1 = 1141,
    增产剂mk2 = 1142,
    增产剂mk3 = 1143,
    机枪弹箱 = 1601,
    钛化弹箱 = 1602,
    超合金弹箱 = 1603,
    炮弹组 = 1604,
    高爆炮弹组 = 1605,
    晶石炮弹组 = 1606,
    等离子胶囊 = 1607,
    反物质胶囊 = 1608,
    导弹组 = 1609,
    超音速导弹组 = 1610,
    引力导弹组 = 1611,
    原型机 = 5101,
    精准无人机 = 5102,
    攻击无人机 = 5103,
    护卫舰 = 5111,
    驱逐舰 = 5112,
    黑雾矩阵 = 5201,
    硅基神经元 = 5202,
    物质重组器 = 5203,
    负熵奇点 = 5204,
    核心素 = 5205,
    能量碎片 = 5206,
    电磁矩阵 = 6001,
    能量矩阵 = 6002,
    结构矩阵 = 6003,
    信息矩阵 = 6004,
    引力矩阵 = 6005,
    宇宙矩阵 = 6006,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum IndirectResource {
    Power,
    Area,
    Time,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub item: Item,
    pub point: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Direct(Cargo),
    Indirect(IndirectResource),
}

#[derive(Clone, Debug)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub num: f64,
}

impl Resource {
    pub const fn from_item(item: Item, num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Direct(Cargo { item, point: 0 }),
            num,
        }
    }

    pub const fn from_item_point(item: Item, point: u64, num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Direct(Cargo { item, point: point }),
            num,
        }
    }

    pub const fn time(num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Indirect(IndirectResource::Time),
            num,
        }
    }

    pub const fn area(num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Indirect(IndirectResource::Area),
            num,
        }
    }

    pub const fn power(num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Indirect(IndirectResource::Power),
            num,
        }
    }
}
