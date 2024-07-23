import React, { useRef } from 'react';
import { CraneProps } from '@/interfaces/CraneProps';

interface NumberInputField {
    name: string;
    label: string;
}

interface InputPanelProps {
    title: string;
    fields: NumberInputField[];
    onSubmit: (data: Record<string, number>) => void
}

const NumberInputPanel: React.FC<InputPanelProps> = ({ title, fields, onSubmit }) => {
    const formRef = useRef<HTMLFormElement>(null);

    const handleSubmit = (event: React.FormEvent) => {
        event.preventDefault();
        const formData = new FormData(event.target as HTMLFormElement);
        const data = Object.fromEntries(
            Array.from(formData.entries()).map(([key, value]) => [key, Number(value)])
        ) as Record<string, number>;
        onSubmit(data);

        if (formRef.current) {formRef.current.reset()};
    };

    return (
        <div className='bg-white rounded-lg shadow-lg p-4 m-2'>
            <h2 className='font-semibold'>{title}</h2>
            <form onSubmit={handleSubmit} ref={formRef}>
                <div className='grid gap-2'>
                    {fields.map((field, idx) => (
                        <div key={idx} className=''>
                            <label className='text-sm font-medium'>{field.label}</label>
                            <input
                                name={field.name}
                                type="number"
                                step="any"
                                required
                                className='w-full rounded-md px-2 shadow border'
                            />
                        </div>
                    ))}
                </div>
                <button type="submit" className='mt-2 w-full bg-blue-500 text-white font-semibold rounded-md py-2'>Submit</button>
            </form>
        </div>
    )
}

const CraneDisplayPanel: React.FC<CraneProps> = ({
    swing_deg,
    lift_mm,
    elbow_deg,
    wrist_deg,
    gripper_mm,
}) => {
    const actuators = [
        { label: 'Swing (deg.)', value: swing_deg },
        { label: 'Lift (mm)', value: lift_mm },
        { label: 'Elbow (deg.)', value: elbow_deg },
        { label: 'Wrist (deg.)', value: wrist_deg },
        { label: 'Gripper (mm)', value: gripper_mm },
    ];
    return(
        <div className="bg-white rounded-lg shadow-lg p-4 m-2">
            <h2 className="font-semibold mb-2">Variable Monitor</h2>
            <div className="space-y-2">
                {actuators.map((actuator, idx) => (
                    <div key={idx} className="flex justify-between">
                        <span>{actuator.label}</span>
                        <span className="font-mono text-blue-500">{actuator.value.toFixed(2)}</span>
                    </div>
                ))}
            </div>
        </div>
    )
}

const ControlPanel: React.FC<{
    crane_props: CraneProps,
    onActuatorSetpointSubmit: (swing: number, lift: number, elbow: number, wrist: number, gripper: number) => void;
    onPositionSubmit: (x: number, y: number, z: number) => void;
}> = ({ crane_props, onActuatorSetpointSubmit: onActuatorSetpointSubmit, onPositionSubmit: onPositionSubmit }) => {
    return (
        <div className="w-full h-full p-2 flex flex-row sm:flex-col bg-lime-600 text-gray-700">
            <CraneDisplayPanel {...crane_props} />
            <NumberInputPanel
                title="Actuator Setpoints"
                fields={[
                    { name: "swing", label: "Swing (deg.)" },
                    { name: "lift", label: "Lift (mm)"},
                    { name: "elbow", label: "Elbow (deg.)" },
                    { name: "wrist", label: "Wrist (deg.)" },
                    { name: "gripper", label: "Gripper (mm)" },
                ]}
                onSubmit={(data) => onActuatorSetpointSubmit(data.swing, data.lift, data.elbow, data.wrist, data.gripper)}
            />
            <NumberInputPanel
                title="Crane Setpoint"
                fields={[
                    { name: "x", label: "x (red)" },
                    { name: "y", label: "y (green)"},
                    { name: "z", label: "z (blue)" },
                ]}
                onSubmit={(data) => onPositionSubmit(data.x, data.y, data.z)}
            />
        </div>
      );
    };

export default ControlPanel