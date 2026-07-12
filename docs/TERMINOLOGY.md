# Cybertarr Terminology

> **Version:** 1.0
> **Status:** Living Document

---

# Purpose

This document defines the official terminology used throughout the Cybertarr project.

The goal is to ensure that research papers, architecture specifications, source code, documentation, and experiments all use consistent language.

Whenever a new architectural concept is introduced, it should first be defined here.

---

# Core Concepts

## Cybertarr

Cybertarr is an experimental developmental cognitive architecture designed to investigate whether intelligence can emerge from simple interacting computational systems that develop over time.

---

## Digital Embryo

The initial stage of Cybertarr.

The Digital Embryo possesses almost no knowledge of the world.

Its purpose is to develop rather than begin fully capable.

Current capabilities include:

* Internal state
* Heartbeat
* Observation

---

## Development Stage

A major milestone in Cybertarr's cognitive evolution.

Current roadmap:

* Embryo
* Infant
* Child
* Adolescent
* Adult

Development stages represent architectural maturity rather than software version numbers.

---

# Environment

## World

The external environment in which the embryo exists.

The World generates observations.

The World is independent of the cognitive architecture.

Examples:

* Simulated world
* Internet world
* Robot world
* Virtual environment

---

## World State

The current condition of the world at a given moment.

Future examples include:

* Time of day
* Temperature
* Weather
* Objects
* Light level
* Sound level

---

## World Tick

A single update cycle of the simulated world.

Each World Tick may change the world's state.

---

# Observation System

## Observation

The smallest unit of experience.

An observation is raw information received from the World.

An observation does not imply understanding.

Example:

Source:

World

Value:

Light

---

## Observation Stream

A continuous sequence of observations produced by the World.

---

## Observer

A software component responsible for collecting observations from the World.

The Observer does not interpret observations.

Its only responsibility is to deliver them to the cognitive architecture.

---

# Cognitive Architecture

## Brain

The primary coordination system of Cybertarr.

The Brain manages internal processing but does not generate external experiences.

---

## Heartbeat

A single execution cycle of the Brain.

Each heartbeat advances the cognitive system.

Example:

Heartbeat

↓

Observe

↓

Update Internal State

↓

Future Processing

---

## Internal State

The dynamic condition of the embryo.

Examples include:

* Energy
* Curiosity
* Uncertainty
* Fatigue
* Confidence

Internal State influences future behavior.

---

# Future Systems

## Memory

A persistent representation formed from meaningful observations.

Memory is not identical to observation.

Observation precedes memory.

---

## Concept

A higher-level abstraction formed from multiple memories.

Concepts represent relationships rather than individual events.

---

## Knowledge

A structured collection of concepts, memories, and relationships.

Knowledge develops gradually through interaction.

---

## Learning

The process by which the cognitive architecture changes based on experience.

Learning may modify memory, concepts, or internal behavior.

---

## Reasoning

The process of producing new conclusions using existing knowledge.

Reasoning depends upon previously acquired information.

---

## Planning

The process of selecting future actions to achieve internal or external goals.

Planning is expected to emerge during later developmental stages.

---

## Goal

A desired future state generated internally or externally.

Goals influence planning and attention.

---

## Curiosity

An internal drive encouraging exploration of unknown information.

Curiosity is expected to influence observation priorities and future learning.

---

# Research Documentation

## CAS

Cybertarr Architecture Specification.

Defines how a subsystem should function before implementation.

---

## ADR

Architecture Decision Record.

Documents important architectural decisions and the reasoning behind them.

---

## Experiment

A controlled procedure designed to evaluate a research hypothesis.

Every experiment should include:

* Objective
* Hypothesis
* Procedure
* Results
* Conclusion

---

## Research Log

A chronological record of the project's scientific progress.

Research logs document:

* Hypotheses
* Experiments
* Observations
* Conclusions
* Open questions

---

# Engineering Terms

## Tick

A single execution cycle.

Examples:

* Brain Tick
* World Tick

Ticks provide temporal progression.

---

## Event

A significant occurrence inside the system.

Examples:

* Observation received
* Memory created
* Goal generated

Events may trigger future processing.

---

## Module

A software component with a clearly defined responsibility.

Modules should communicate through explicit interfaces.

---

## Architecture

The organization and interaction of all systems within Cybertarr.

Architecture defines behavior independently of implementation details.

---

# Guiding Rule

Whenever a new concept is introduced into Cybertarr, it should first be defined in this document before it appears in code, experiments, or research papers.

Consistent terminology is essential for reproducible research, maintainable software, and clear scientific communication.
