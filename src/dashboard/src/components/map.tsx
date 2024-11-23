// File: src/components/MapWithEmojis.tsx
import { useState } from "react";
import * as React from "react";
import { MapContainer, TileLayer, useMapEvents, Marker } from "react-leaflet";
import * as L from "leaflet"; // Import Leaflet explicitly
import "leaflet/dist/leaflet.css";
import { MUNICH_X_CENTER, MUNICH_Y_CENTER} from "@/config.ts";
import { randomHumanEmoji } from "@/lib/utils.ts";

const emojiIcon = (emoji: string) =>
    new L.DivIcon({
        html: emoji,
        className: "text-3xl",
        iconSize: [24, 24],
        iconAnchor: [12, 12],
    });

const AddEmojis = ({ onAddEmoji }: { onAddEmoji: (lat: number, lng: number) => void }) => {
    useMapEvents({
        click(e) {
            onAddEmoji(e.latlng.lat, e.latlng.lng);
        },
    });
    return null;
};

const Map = () => {
    const [markers, setMarkers] = useState<{ lat: number; lng: number; emoji: string }[]>([]);

    const handleAddEmoji = (lat: number, lng: number) => {
        setMarkers((prev) => [...prev, { lat, lng, emoji: randomHumanEmoji() }]); // Default emoji is a car
    };

    return (
        // <div className="padded-left-half">
            <MapContainer center={[MUNICH_X_CENTER , MUNICH_Y_CENTER]} zoom={12} className="h-full w-full rounded-md shadow-md">
                <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
                <AddEmojis onAddEmoji={handleAddEmoji} />
                {markers.map((marker, index) => (
                    <Marker
                        key={index}
                        position={[marker.lat, marker.lng]}
                        icon={emojiIcon(marker.emoji)}
                    />
                ))}
            </MapContainer>
        // </div>
    );
};

export default Map;
