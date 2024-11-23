import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function randomHumanEmoji() {
  const humanEmojis = [
    ["👨", "👨🏻", "👨🏼", "👨🏽", "👨🏾", "👨🏿"], // Man
    ["👩", "👩🏻", "👩🏼", "👩🏽", "👩🏾", "👩🏿"], // Woman
    ["🧑", "🧑🏻", "🧑🏼", "🧑🏽", "🧑🏾", "🧑🏿"], // Person
    ["👮", "👮🏻", "👮🏼", "👮🏽", "👮🏾", "👮🏿"], // Police Officer
    ["🕵️", "🕵🏻", "🕵🏼", "🕵🏽", "🕵🏾", "🕵🏿"], // Detective
    ["💂", "💂🏻", "💂🏼", "💂🏽", "💂🏾", "💂🏿"], // Guard
    ["👷", "👷🏻", "👷🏼", "👷🏽", "👷🏾", "👷🏿"], // Construction Worker
    ["👩‍🌾", "👩🏻‍🌾", "👩🏼‍🌾", "👩🏽‍🌾", "👩🏾‍🌾", "👩🏿‍🌾"], // Woman Farmer
    ["👨‍🌾", "👨🏻‍🌾", "👨🏼‍🌾", "👨🏽‍🌾", "👨🏾‍🌾", "👨🏿‍🌾"], // Man Farmer
    ["👩‍🍳", "👩🏻‍🍳", "👩🏼‍🍳", "👩🏽‍🍳", "👩🏾‍🍳", "👩🏿‍🍳"], // Woman Cook
    ["👨‍🍳", "👨🏻‍🍳", "👨🏼‍🍳", "👨🏽‍🍳", "👨🏾‍🍳", "👨🏿‍🍳"], // Man Cook
    ["👩‍🎓", "👩🏻‍🎓", "👩🏼‍🎓", "👩🏽‍🎓", "👩🏾‍🎓", "👩🏿‍🎓"], // Woman Student
    ["👨‍🎓", "👨🏻‍🎓", "👨🏼‍🎓", "👨🏽‍🎓", "👨🏾‍🎓", "👨🏿‍🎓"], // Man Student
    ["👩‍🔬", "👩🏻‍🔬", "👩🏼‍🔬", "👩🏽‍🔬", "👩🏾‍🔬", "👩🏿‍🔬"], // Woman Scientist
    ["👨‍🔬", "👨🏻‍🔬", "👨🏼‍🔬", "👨🏽‍🔬", "👨🏾‍🔬", "👨🏿‍🔬"], // Man Scientist
    ["👩‍💻", "👩🏻‍💻", "👩🏼‍💻", "👩🏽‍💻", "👩🏾‍💻", "👩🏿‍💻"], // Woman Technologist
    ["👨‍💻", "👨🏻‍💻", "👨🏼‍💻", "👨🏽‍💻", "👨🏾‍💻", "👨🏿‍💻"], // Man Technologist
    ["🧑‍💻", "🧑🏻‍💻", "🧑🏼‍💻", "🧑🏽‍💻", "🧑🏾‍💻", "🧑🏿‍💻"], // Technologist
    ["👩‍🎤", "👩🏻‍🎤", "👩🏼‍🎤", "👩🏽‍🎤", "👩🏾‍🎤", "👩🏿‍🎤"], // Woman Singer
    ["👨‍🎤", "👨🏻‍🎤", "👨🏼‍🎤", "👨🏽‍🎤", "👨🏾‍🎤", "👨🏿‍🎤"], // Man Singer
    ["👩‍🎨", "👩🏻‍🎨", "👩🏼‍🎨", "👩🏽‍🎨", "👩🏾‍🎨", "👩🏿‍🎨"], // Woman Artist
  ].flatMap((emoji) => emoji);

  return humanEmojis[Math.floor(Math.random() * humanEmojis.length)];
}
