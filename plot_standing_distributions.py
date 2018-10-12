"""
Plot distribution of season standings.
"""

import os
import argparse
import matplotlib.pyplot as plt
import numpy as np

def plot_distribution(file_name, team_name, distribution, density=True):
    """Plot bar chart of distributions and save the figure"""

    if density:
        distribution = [x / np.sum(distribution) for x in distribution]

    inds = np.arange(len(distribution))

    plt.style.use("ggplot")
    plt.figure()
    plt.bar(inds, distribution, width=0.8)
    plt.title(team_name)
    plt.ylabel("Probability")
    plt.xlabel("Position")
    plt.ylim((0.0, 1.1))
    plt.xticks(inds, inds + 1)
    plt.savefig(file_name, dpi=150)

def main():
    """Main"""

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input_fname", required=True,
                        help="Input csv file with season standing distributions for every team")
    parser.add_argument("-t", "--teams", default="all", nargs="*",
                        help="Name of team, default: all")
    parser.add_argument("-o", "--output_dir", default=os.getcwd(),
                        help="Output directory to put plot(s)")
    args = parser.parse_args()

    position_distributions = {}

    with open(args.input_fname, "r") as ifile:
        content = ifile.readlines()
    content = [c.rstrip() for c in content]

    headers = [int(x.split("Pos ")[-1].replace("\"", "")) for x in content[0].split(",")[1:]]
    for team_distribution_str in content[1:]:
        team_distribution = team_distribution_str.split(",")
        position_distributions[team_distribution[0]] = [np.uint64(x) for x in team_distribution[1:]]

    header_str = "{:<15}".format("Team")
    for val in headers:
        header_str += "{:>13}".format(val)
    print(header_str)
    for team_name, distribution in position_distributions.items():
        if team_name in args.teams or "all" in args.teams:
            team_name = team_name.replace("\"", "")
            distribution_str = "{:<15}".format(team_name)
            for val in distribution:
                distribution_str += "{:>13}".format(val)
            print(distribution_str)
            team_fname = team_name.replace(" ", "_").lower() + ".png"
            output_filename = os.path.join(args.output_dir, team_fname)
            plot_distribution(output_filename, team_name, distribution)

if __name__ == "__main__":
    main()
