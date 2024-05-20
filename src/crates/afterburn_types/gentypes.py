import os
import re

os.system("cargo test")

enum_regex = re.compile(r"export interface \w+ (\".*\")")
type_regex = re.compile(r"(export type .*;)")

bind_dir = os.path.join(os.getcwd(), "bindings")
with open("../../../../app-frontend/src/types.d.ts", "w") as t_file:
    for filename in os.listdir(bind_dir):
        with open(os.path.join(bind_dir, filename), 'r') as f:
            content = type_regex.findall(f.read()).pop()

            content = content.replace("type", "interface")
            content = content.replace("= ", "")
            if enum_regex.match(content):
                content = content.replace("\"", "{ ", 1)
                content = content.replace("export interface", "export enum")
                content = content.replace("\"", "")
                content = content.replace(" | ", ", ")
                content = content.replace(";", " };")

            content = content.replace(";", "")
            t_file.write(f"{content}\n")
