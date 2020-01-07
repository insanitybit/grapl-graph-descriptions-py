from setuptools import setup, find_packages

setup(
    name="grapl_graph_descriptions_py",
    version="0.1.0",
    description="Library for describe Grapl graphs",
    url="https://github.com/insanitybit/grapl-graph-descriptions-py/",
    author="insanitybit",
    author_email="insanitybit@gmail.com",
    license="MIT",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
    ],
    zip_safe=False,
    packages=find_packages(),
    package_data={
        'grapl_graph_descriptions_py': ["py.typed"],
    },
    include_package_data=True,
    install_requires=['protobuf'],
)
