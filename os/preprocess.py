import os

# 设置你想要写入的文件名
output_filename = "combined_files.txt"

def combine_files(output_path):
    with open(output_path, "w", encoding="utf-8") as output_file:
        # os.walk遍历当前目录及所有子目录的所有文件
        for root, dirs, files in os.walk("."):
            # 检查当前目录路径是否包含不需要遍历的目录
            if "./target" in root:
                continue  # 跳过这个目录和其子目录

            for file in files:
                # 拼接完整的文件路径
                path = os.path.join(root, file)
                # 写入文件路径注释
                output_file.write(f"# {path}\n")
                try:
                    # 打开并读取文件内容，然后写入
                    with open(path, "r", encoding="utf-8") as file_content:
                        output_file.write(file_content.read() + "\n")
                except Exception as e:
                    # 如果文件打开或读取有误，则写入一个错误消息
                    output_file.write(f"# Error reading file {path}: {e}\n")
                # 在文件之间添加一个分隔符
                output_file.write("\n\n")

# 调用函数，传入你希望输出的文件路径
combine_files(output_filename)
