"""
Plot distribution of season standings.
"""

import os
import argparse
import matplotlib.pyplot as plt
import numpy as np

def plot_distribution(file_name, team_name, distribution, density=True, tick_labels=None):
    """Plot bar chart of distributions and save the figure"""

    if density:
        distribution = [x / np.sum(distribution) for x in distribution]

    inds = np.arange(len(distribution))

    plt.style.use("ggplot")
    plt.figure()
    plt.bar(inds, distribution, width=0.8)
    plt.title(team_name)
    plt.ylabel("Probability")
    plt.ylim((0.0, 1.1))
    if tick_labels is None:
        plt.xlabel("Position")
        tick_labels = inds + 1
        plt.xticks(inds, tick_labels)
    else:
        plt.xticks(inds, tick_labels, rotation=90)
        plt.margins(0.2)
        plt.subplots_adjust(bottom=0.3)
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

    for pos in range(1, len(position_distributions.values())+1):
        ind = pos - 1
        team_names = []
        position_distribution = []
        for team_name, distribution in position_distributions.items():
            if distribution[ind] > 0.0:
                team_names.append('{}'.format(team_name))
                position_distribution.append(distribution[ind])
        team_names = [x for _, x in sorted(zip(position_distribution, team_names), reverse=True)]
        position_distribution = sorted(position_distribution, reverse=True)
        pos_fname = "position_{:02}.png".format(pos)
        output_filename = os.path.join(args.output_dir, pos_fname)
        plot_distribution(output_filename, "Position {:02}".format(pos),
                          position_distribution, tick_labels=team_names)

if __name__ == "__main__":
    main()
