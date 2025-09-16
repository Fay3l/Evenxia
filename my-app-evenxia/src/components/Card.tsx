type CardProps = {
    readonly title?: string;
    readonly description?: string;
    readonly imageUrl?: string;
    readonly start_date?: string;
    readonly end_date?: string;
    readonly address?: string;

    readonly onButtonClick?: () => void;
};
export default function Card({title="",description="",imageUrl="", onButtonClick}: CardProps) {
    return (
    <div className="card bg-base-100  shadow-sm">
        <figure>
            <img
                src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
                alt="Shoes" />
        </figure>
        <div className="card-body">
            <h2 className="card-title">{title}</h2>
            <p>{description}</p>
            <div className="card-actions justify-end">
                <button onClick={onButtonClick} className="btn btn-primary">Voir plus en détails</button>
            </div>
        </div>
    </div>)
}