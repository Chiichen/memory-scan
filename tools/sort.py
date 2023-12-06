import csv
import argparse

def sort_csv_file(input_file, output_file):
    # Read the CSV file
    with open(input_file, 'r') as file:
        lines = list(csv.reader(file))
    
    # Sort the lines based on the key in ascending order
    sorted_lines = sorted(lines, key=lambda x: int(x[0]))
    print(len(sorted_lines))
    # Calculate the threshold for filtering
    max_key = max(sorted_lines, key=lambda x: int(x[1]))
    print(max_key)
    max_value = int(max_key[1])
    # max_value = int(sorted_lines[-1][0])
    threshold = max_value * 0.05

    # Filter out data points less than 10% of the maximum value
    filtered_lines = [line for line in sorted_lines if int(line[1]) >= threshold]
    print(len(filtered_lines))
    # Write the filtered lines to a new CSV file
    with open(output_file, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(filtered_lines)

# 创建命令行参数解析器
parser = argparse.ArgumentParser(description='Generate a bar chart from a CSV file.')
parser.add_argument('csv_file', type=str, help='Path to the CSV file')

# 解析命令行参数
args = parser.parse_args()

sort_csv_file(args.csv_file, 'sorted_csv.csv')