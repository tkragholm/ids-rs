from setuptools import setup, find_packages
from setuptools_rust import RustBin

setup(
    name="ids-rs",
    version="0.1.0",
    description="Incidence Density Sampling CLI Tool",
    author="Tobias Kragholm",
    author_email="tkragholm@gmail.com",
    classifiers=[
        "Programming Language :: Rust",
        "Programming Language :: Python :: 3",
        "Environment :: Console",
        "Intended Audience :: Science/Research",
    ],
    python_requires=">=3.7",
    rust_extensions=[
        RustBin(
            "ids",  # Target name from Cargo.toml
            path="Cargo.toml",
            debug=False  # Use release profile
        )
    ],
    packages=find_packages(where="python"),
    package_dir={"": "python"},
    include_package_data=True,
    zip_safe=False,
    # Add entry point for the command
    entry_points={
        "console_scripts": [
            "ids=ids:main",
        ],
    },
)