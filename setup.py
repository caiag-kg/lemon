from setuptools import setup, find_packages


setup(
    name='lemon',
    version='0.1.0b1',
    packages=find_packages(),
    install_requires=[
        'click',
    ],
    entry_points={
        'console_scripts': [
            'lemon = lemon.main:main',
        ],
    },
)