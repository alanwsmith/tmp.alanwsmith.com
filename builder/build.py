#!/usr/bin/env python3

import os

from datetime import datetime
from string import Template

class Builder():
    def __init__(self):
        self.project_root = os.path.dirname(os.path.dirname(__file__))
        self.source_root = f"{self.project_root}/builder/src"
        self.site_root = f"{self.project_root}/site"
        self.parts = {}

    def load_template(self):
        with open(f"{self.source_root}/TEMPLATE.html") as _template:
            self.template = _template.read()

    def build_content(self):
        # Make dynamic content here
        # self.parts['CONTENT'] = "the quick brown fox"
        pass

    def load_parts(self):
        for file_part in self.file_parts:
            with open(f"{self.source_root}/{file_part}") as _file_part:
                name_parts = file_part.split('.')
                self.parts[name_parts[0]] = _file_part.read()

    def output_file(self):
        with open(f"{self.site_root}/index.html", 'w') as _out:
            _out.write(self.template)

    def make_page(self, template_path, output_path, data):
        with open(template_path) as _template:
            template = Template(_template.read())
            with open(output_path, 'w') as _output:
                _output.write(
                    template.substitute(data)
                )

if __name__ == "__main__":
    b = Builder()
    b.load_template()
    b.file_parts = ['HEAD.html', 'BODY.html', 'CSS.css', 'JS.js']
    b.load_parts()
    b.build_content()
    b.make_page(
        f"{b.source_root}/TEMPLATE.html",
        f"{b.site_root}/index.html",
        b.parts
    )

    print(f"## Completed Build: {datetime.now()}")
