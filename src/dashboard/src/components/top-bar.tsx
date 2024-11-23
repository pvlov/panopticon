import * as React from "react";
import { ModeToggle } from "@/components/mode-toggle";

const TopBar: React.FC = () => {
    return (
        <div className="top-bar">
            <div className="top-bar-content">
                <ModeToggle/>
            </div>
        </div>
    );
};

export default TopBar;