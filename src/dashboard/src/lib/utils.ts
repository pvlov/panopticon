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

export function randomVehicle() {
  const carEmojis = [
    "🚗", // Red Car
    "🚕", // Taxi
    "🚙", // SUV
    "🚌", // Bus
    "🚎", // Trolleybus
    "🏎️", // Racing Car
    "🚓", // Police Car
    "🚑", // Ambulance
    "🚒", // Fire Truck
    "🚐", // Minibus
    "🚚", // Delivery Truck
    "🚛", // Articulated Lorry
    "🚜", // Tractor
    "🛻", // Pickup Truck
    "🚍", // Oncoming Bus
    "🚔", // Oncoming Police Car
    "🚖", // Oncoming Taxi
    "🚘", // Oncoming Automobile
    "🚡", // Aerial Tramway
    "🚠", // Mountain Cableway
    "🚟", // Suspension Railway
    "🚃", // Railway Car
    "🚋", // Tram Car
    "🚝", // Monorail
    "🚞", // Mountain Railway
    "🚄", // High-Speed Train
    "🚅", // Bullet Train
    "🚈", // Light Rail
    "🚂", // Locomotive
    "🚆", // Train
    "🚇", // Metro
    "🚊", // Tram
    "🚉", // Station
  ];

  return carEmojis[Math.floor(Math.random() * carEmojis.length)];
}
