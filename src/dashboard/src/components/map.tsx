import * as React from "react";
import { MapContainer, TileLayer, Marker, Polyline } from "react-leaflet";
import * as L from "leaflet";
import "leaflet/dist/leaflet.css";
import { MUNICH_X_CENTER, MUNICH_Y_CENTER } from "@/config.ts";
import { randomHumanEmoji } from "@/lib/utils.ts";
import { Customer, Vehicle } from "@/model/models.ts";
import { emojiMap } from "@/lib/utils.ts";

const emojiIcon = (emoji: string) =>
    new L.DivIcon({
        html: emoji,
        className: "text-3xl",
        iconSize: [24, 24],
        iconAnchor: [12, 12],
    });

const Map = ({ customers, vehicles }: { customers: Customer[], vehicles: Vehicle[] }) => {

    const pairs = [];

    for (const customer of customers) {
        if (!emojiMap.has(customer.id)) {
            emojiMap.set(customer.id, randomHumanEmoji());
        }
    }

    for (const vehicle of vehicles) {
        if (vehicle.customerId) {
            const customer = customers.find((c) => c.id === vehicle.customerId);
            if (customer) {
                pairs.push({
                    customer: customer,
                    vehicle: vehicle
                });
            }
        }
    }


    return (
        <MapContainer center={[MUNICH_X_CENTER, MUNICH_Y_CENTER]} zoom={12} className="h-full w-full rounded-md shadow-md">
            <TileLayer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png" />
            {vehicles.map((vehicle, index) => (
                <Marker
                    key={`vehicle-${index}`}
                    position={[vehicle.coordX, vehicle.coordY]}
                    icon={emojiIcon( "🚕")}
                />
            ))}
            {customers.map((customer, index) => (
                <Marker
                    key={`customer-${index}`}
                    position={[customer.coordX, customer.coordY]}
                    icon={emojiIcon(emojiMap.get(customer.id) || "👤")}
                />
            ))}
            {pairs.map((pair, index) => {
                return (
                    <Polyline
                        key={`line-${index}`}
                        positions={[
                            [pair.customer.coordX, pair.customer.coordY],
                            [pair.vehicle.coordX, pair.vehicle.coordY]
                        ]}
                        color="rgba(128, 128, 128, 0.8)"
                        dashArray="5, 5" // Creates a dashed pattern
                    />
                );
            })}
        </MapContainer>
    );
};

export default Map;