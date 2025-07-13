import useMyOrders from "../hooks/useMyOrders";

export default function MyOrders() {
  const { myOrders, cancelOrder } = useMyOrders();

  return (
    <div className="p-4 rounded shadow bg-white mt-4 hover:shadow-lg transition-shadow">
      <h2 className="font-bold text-lg mb-2">My Orders</h2>
      {myOrders.length === 0 ? (
        <p className="text-gray-500">No active orders.</p>
      ) : (
        <ul className="space-y-2">
          {myOrders.map((order) => (
            <li
              key={order.id}
              className="flex justify-between items-center border p-2 rounded"
            >
              <div>
                <span
                  className={`font-semibold ${
                    order.side === "buy" ? "text-green-600" : "text-red-600"
                  }`}
                >
                  {order.side.toUpperCase()}
                </span>{" "}
                @ £{order.price} × {order.quantity}
              </div>
              <button
                onClick={() => cancelOrder(order.id)}
                className="text-sm text-white bg-red-500 px-2 py-1 rounded"
              >
                Cancel
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
