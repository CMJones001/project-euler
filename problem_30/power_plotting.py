#!/usr/bin/env python

"""
Plot patterns in the sum of digits to a given power.
"""

import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def main():
    power_test = Path("power_values.txt")

    if power_test.is_file():
        print("File exists")
        power_values = np.loadtxt(power_test, dtype=np.uint64)
        print(power_values)

    else:
        raise ValueError("File does not exist")

    fig, axs = plt.subplots(nrows=2, figsize=(8, 8))
    num_scale = np.arange(len(power_values)) + 2

    for ax in axs:
        ax.scatter(num_scale, power_values, marker='.', s=2, alpha=0.1)
        ax.plot(num_scale, num_scale, linestyle='--', linewidth=0.5, color='black')
        ax.set_aspect('equal')

    axs[0].set_xscale('log')
    axs[0].set_yscale('log')

    plt.tight_layout()
    plt.savefig("/tmp/power_plot.png", dpi=220, bbox_inches='tight')

if __name__ == "__main__":
    main()
