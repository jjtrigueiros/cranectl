import React, { useRef } from 'react';

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
        <div className='border rounded-md px-2 py-2'>
            <h2>{title}</h2>
            <form onSubmit={handleSubmit}>
                {fields.map((field, idx) => (
                    <div key={idx}>
                        <label>{field.label}</label>
                        <input
                            type="number"
                            step="any"
                            name={field.name}
                            required
                            className='rounded-md px-2 text-black'
                        />
                    </div>
                ))}
                <button type="submit" className='mt-2 w-full bg-blue-500 text-white font-semibold rounded-md'>Submit</button>
            </form>
        </div>
    )
}

const ControlPanel: React.FC<{
    onSpeedSubmit: (swing: number, lift: number, elbow: number, wrist: number, gripper: number) => void;
}> = ({ onSpeedSubmit: onSpeedSubmit }) => {
    return (
        <div className="w-full h-full p-4 flex flex-row md:flex-col bg-lime-600">
            <NumberInputPanel
                title="Speed Control"
                fields={[
                    { name: "swing", label: "Swing (deg.)" },
                    { name: "lift", label: "Lift (mm)"},
                    { name: "elbow", label: "Elbow (deg.)" },
                    { name: "wrist", label: "Wrist (deg.)" },
                    { name: "gripper", label: "Gripper (mm)" },
                ]}
                onSubmit={(data) => onSpeedSubmit(data.swing, data.lift, data.elbow, data.wrist, data.gripper)} />
        </div>
      );
    };

export default ControlPanel