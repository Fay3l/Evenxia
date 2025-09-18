type CardProps = {
    readonly title?: string;
    readonly description?: string;
    readonly imageUrl?: string;
    readonly start_date?: string;
    readonly end_date?: string;
    readonly address?: string;
    readonly onButtonClick?: () => void;
};

export default function Card({ title = "", description = "", imageUrl = "", address = "", start_date = "", end_date = "", onButtonClick }: CardProps) {
    return (
        <div className="card bg-base-100  shadow-sm">
            <figure>
                <img
                    className=""
                    src={imageUrl}
                    alt="Event" />
            </figure>
            <div className="card-body flex flex-col justify-between">
                <h2 className="card-title">{title}</h2>
                <div className="flex flex-col gap-2 justify-around">
                    <p>{description}</p>
                </div>
                <div className="card-actions justify-end max-sm:justify-none">
                    <div className="flex flex-row justify-between items-center w-full max-sm:flex-col gap-2">
                        <p className="font-bold">{start_date}</p>
                        <button onClick={onButtonClick} className="btn btn-primary max-sm:btn-sm p-4">Voir plus en détails</button>
                    </div>  
                    <div className="max-sm:text-sm">
                        <p className="italic">{address}</p>
                    </div>
                </div>
            </div>
        </div>)
}