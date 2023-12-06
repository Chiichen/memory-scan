import argparse
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
# def hex_format(x, pos):
#     # 将整数转换为十六进制字符串
#     return str(hex(x))


def generate_bar_chart(csv_file):
    # 读取 CSV 文件
    data = pd.read_csv(csv_file, header=None, names=['Key', 'Value'],dtype = {'Key' : int,'Value':int}).sort_values(by="Key",ascending=True)
    
    data['Column'] = 1

    # 绘制柱状图
    pivot_table = data.pivot_table(values='Value', index='Key', columns='Column')

    # # 计算value的20%阈值
    # threshold = pivot_table.values.max() * 0.2

    # # 过滤掉value低于20%阈值的数据
    # filtered_pivot_table = pivot_table[pivot_table.values >= threshold]

    filtered_pivot_table = pivot_table

    plt.figure(figsize=(12,12))

    # 绘制热力图
    sns.heatmap(filtered_pivot_table, cmap='Reds')

    # 获取y轴刻度的数量
    num_ticks = filtered_pivot_table.shape[0]
    
    # 生成20个均匀刻度
    yticks = np.linspace(0, num_ticks - 1, 20, dtype=int)

    max_label = filtered_pivot_table.index.max()
    ylabels = np.linspace(0, max_label - 1, 20, dtype=int)
    # 设置刻度位置和标签
    plt.yticks(yticks, [hex(i) for i in ylabels])

    plt.xlabel('Time/sec')
    plt.ylabel('Address Space')
    plt.title('Heatmap')

    plt.savefig('./map.eps')

# 创建命令行参数解析器
parser = argparse.ArgumentParser(description='Generate a bar chart from a CSV file.')
parser.add_argument('csv_file', type=str, help='Path to the CSV file')

# 解析命令行参数
args = parser.parse_args()

# 调用函数生成柱状图
generate_bar_chart(args.csv_file)