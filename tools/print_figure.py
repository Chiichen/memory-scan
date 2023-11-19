import argparse
from matplotlib import ticker
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def hex_format(x, pos):
    # 将整数转换为十六进制字符串
    return f'{int(x):X}'


def generate_bar_chart(csv_file):
    # 读取 CSV 文件
    data = pd.read_csv(csv_file, header=None, names=['Key', 'Value'])

    data['Column'] = 1

    # 绘制柱状图
    pivot_table = data.pivot_table(values='Value', index='Key', columns='Column').sort_index(
    axis=0, ascending=False)

    # 计算value的95%阈值
    threshold = pivot_table.values.max() * 0.05
    print(pivot_table)
    # 过滤掉value低于95%阈值的数据
    filtered_pivot_table = pivot_table[pivot_table.values >= threshold]


    # 绘制热力图
    sns.heatmap(filtered_pivot_table, cmap='Reds')

    # 创建自定义刻度格式化器
    formatter = ticker.FuncFormatter(hex_format)

    # 应用自定义刻度格式化器到纵坐标轴
    plt.gca().yaxis.set_major_formatter(formatter)
    plt.xlabel('Time/sec')
    plt.ylabel('Address Space')
    plt.title('Heatmap')
    plt.savefig('./map.svg')
    # plt.show()

# 创建命令行参数解析器
parser = argparse.ArgumentParser(description='Generate a bar chart from a CSV file.')
parser.add_argument('csv_file', type=str, help='Path to the CSV file')

# 解析命令行参数
args = parser.parse_args()

# 调用函数生成柱状图
generate_bar_chart(args.csv_file)