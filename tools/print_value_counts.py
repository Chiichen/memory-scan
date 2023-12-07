from math import log
import sys
import csv
import matplotlib.pyplot as plt
import matplotlib.pyplot as plt
from matplotlib.patches import  ConnectionPatch
import numpy as np

def zone_and_linked(ax,axins,zone_left,zone_right,x,y,linked='bottom',
                    x_ratio=0.05,y_ratio=0.05):
    """缩放内嵌图形，并且进行连线
    ax:         调用plt.subplots返回的画布。例如:fig,ax = plt.subplots(1,1)
    axins:      内嵌图的画布。 例如 axins = ax.inset_axes((0.4,0.1,0.4,0.3))
    zone_left:  要放大区域的横坐标左端点
    zone_right: 要放大区域的横坐标右端点
    x:          X轴标签
    y:          列表,所有y值
    linked:     进行连线的位置，{'bottom','top','left','right'}
    x_ratio:    X轴缩放比例
    y_ratio:    Y轴缩放比例
    """
    xlim_left = x[zone_left]-(x[zone_right]-x[zone_left])*x_ratio
    xlim_right = x[zone_right]+(x[zone_right]-x[zone_left])*x_ratio

    y_data = np.hstack([yi[zone_left:zone_right] for yi in y])
    ylim_bottom = np.min(y_data)-(np.max(y_data)-np.min(y_data))*y_ratio
    ylim_top = np.max(y_data)+(np.max(y_data)-np.min(y_data))*y_ratio

    axins.set_xlim(xlim_left, xlim_right)
    axins.set_ylim(ylim_bottom, ylim_top)

    
    if linked == 'right-middle':
        xyA_1, xyB_1 = (xlim_left,ylim_top), (xlim_right,(ylim_top+ylim_bottom)/2)
        con = ConnectionPatch(xyA=xyA_1,xyB=xyB_1,coordsA="data",
                          coordsB="data",axesA=axins,axesB=ax)
        axins.add_artist(con)
        return
        

    ax.plot([xlim_left,xlim_right,xlim_right,xlim_left,xlim_left],
            [ylim_bottom,ylim_bottom,ylim_top,ylim_top,ylim_bottom],"black")

    if linked == 'bottom':
        xyA_1, xyB_1 = (xlim_left,ylim_top), (xlim_left,ylim_bottom)
        xyA_2, xyB_2 = (xlim_right,ylim_top), (xlim_right,ylim_bottom)
    elif  linked == 'top':
        xyA_1, xyB_1 = (xlim_left,ylim_bottom), (xlim_left,ylim_top)
        xyA_2, xyB_2 = (xlim_right,ylim_bottom), (xlim_right,ylim_top)
    elif  linked == 'left':
        xyA_1, xyB_1 = (xlim_right,ylim_top), (xlim_left,ylim_top)
        xyA_2, xyB_2 = (xlim_right,ylim_bottom), (xlim_left,ylim_bottom)
    elif  linked == 'right':
        xyA_1, xyB_1 = (xlim_left,ylim_top), (xlim_right,ylim_top)
        xyA_2, xyB_2 = (xlim_left,ylim_bottom), (xlim_right,ylim_bottom)

    con = ConnectionPatch(xyA=xyA_1,xyB=xyB_1,coordsA="data",
                          coordsB="data",axesA=axins,axesB=ax)
    axins.add_artist(con)
    con = ConnectionPatch(xyA=xyA_2,xyB=xyB_2,coordsA="data",
                          coordsB="data",axesA=axins,axesB=ax)
    axins.add_artist(con)



def plot_value_counts(csv_file):
    # Read the CSV file
    with open(csv_file, 'r') as file:
        lines = list(csv.reader(file))

    # Extract values from CSV and count their occurrences
    values = [int(line[1]) for line in lines]
    value_counts = {}
    for value in values:
        value_counts[value] = value_counts.get(value, 0) + 1
    # Sort the values and their counts in ascending order
    frequency_counts = sorted(value_counts.items(),key=lambda x: x[1])
    print(frequency_counts)
    filtered_counts = frequency_counts[:int(len(frequency_counts)*0.99)]
    print(max(filtered_counts, key=lambda x: int(x[1])))
    sorted_counts = sorted(filtered_counts, key=lambda x: x[0])
    x = [count[0] for count in sorted_counts]
    y = [count[1] for count in sorted_counts]
    
    # max_value = max(sorted_counts,key=lambda x:int(x[1]))[1]
    # max_threshold = int(max_value * 0.999)
    # filtered_counts = [line for line in sorted_counts if int(line[1]) <= max_threshold ]

    # print(max(filtered_counts, key=lambda x: int(x[1])))
    fig, ax = plt.subplots(1,1)
    # Plot the curve
    ax.plot(x, y)
    plt.xlabel('Counts')
    plt.ylabel('Access Frequency')

    plt.title('Memory access Frequency')
    # 绘制缩放图
    axins = ax.inset_axes((0.4, 0.1, 0.4, 0.3))

    # 在缩放图中也绘制主图所有内容，然后根据限制横纵坐标来达成局部显示的目的
    axins.plot(x,y)
    
    # 局部显示并且进行连线
    zone_and_linked(ax, axins, 0, 30, x ,[y], 'right-middle')


    plt.savefig('./count.eps')

def plot_logvalue_counts(csv_file):
    # Read the CSV file
    with open(csv_file, 'r') as file:
        lines = list(csv.reader(file))

    # Extract values from CSV and count their occurrences
    values = [int(line[1]) for line in lines]
    value_counts = {}
    for value in values:
        value_counts[value] = value_counts.get(value, 0) + 1
    # Sort the values and their counts in ascending order
    frequency_counts = sorted(value_counts.items(),key=lambda x: x[1])
    print(frequency_counts)
    filtered_counts = frequency_counts[:]
    print(max(filtered_counts, key=lambda x: int(x[1])))
    sorted_counts = sorted(filtered_counts, key=lambda x: x[0])
    x = list(range(0,len(sorted_counts)))
    y = [int(log(x)) for x in [count[1] for count in sorted_counts]]
     
    # max_value = max(sorted_counts,key=lambda x:int(x[1]))[1]
    # max_threshold = int(max_value * 0.999)
    # filtered_counts = [line for line in sorted_counts if int(line[1]) <= max_threshold ]

    # print(max(filtered_counts, key=lambda x: int(x[1])))
    fig, ax = plt.subplots(1,1)
    # Plot the curve
    ax.scatter(x, y)
    plt.xlabel('Counts')
    plt.ylabel('Access Frequency')

    plt.title('Memory access Frequency')

    plt.savefig('./logcount.eps')

# Get the CSV file path from command line argument
if len(sys.argv) > 1:
    csv_file_path = sys.argv[1]
    plot_value_counts(csv_file_path)
    plot_logvalue_counts(csv_file_path)
else:
    print("Please provide the path to the CSV file as a command line argument.")

