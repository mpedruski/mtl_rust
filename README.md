# mtl_map?

## Introduction

I spent a good deal of the winter of 2020 in Toronto missing Montréal. I came home to Montréal just before the pandemic launched itself on Québec, with the end result that I was back in Montréal but I couldn't go to many of the places in Montréal that spelled Montréal for me. To be honest, I'm not even sure some of those places still exist, given how the world has changed since then.

To compensate for the melancholy that comes with being cooped up I've made a tour
of important places in Montréal that users can take to experience different locations
on the city, and hear why they're important to me.

This version of the tour is actually a rewrite of the Python version in my MTL_Map repository. The rewrite offered me a chance to refactor some of the code and deploy my first project in Rust while generating a binary that would be easier to share.

## Vision

My vision for the project involves three separate ways of navigating Montréal:

* A spatial walking tour that allows users to move through Montréal a bit like someone might move through it while walking. The user can choose to move north, south, east, west, to the closest location, or to a random location.

* An temporal walking tour that allows users to move forward or backward in time, as well as visits to random locations from each of the four seasons.

* An 'emotional' walking tour that doesn't link spaces by physical or temporal proximity, but by how they are linked in memory. The goal would be a bit to present the tour of Montréal a bit as if one were reading *À la recherche du temps perdu* or *Mrs. Dalloway*. This hasn't been implemented for the Rust version yet.

Ultimately I would like the project to have significant visual and textual elements:

* A map showing users where they are in MTL, and what other locations are available (along with some animation showing the trip between the two to make the transitions more felt).

* Illustrations of each location

* Text accompanying each location. This is the only aspect currently functional.

## Current status

Currently the project offers a text-only spatial or temporal tour.
