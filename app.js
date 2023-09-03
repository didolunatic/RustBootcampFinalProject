document.addEventListener('DOMContentLoaded', async () => {
    const contract = new ink.Client("YOUR_CONTRACT_ADDRESS"); // Replace with the actual contract address

    const listItemForm = document.getElementById('list-item-form');
    const bidForm = document.getElementById('bid-form');
    const itemDetails = document.getElementById('item-details');

    listItemForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const description = document.getElementById('item-description').value;
        await contract.list_item(description);
        alert('Item listed successfully!');
    });

    bidForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const itemId = parseInt(document.getElementById('item-id').value, 10);
        const bidAmount = parseInt(document.getElementById('bid-amount').value, 10);
        await contract.bid(itemId, bidAmount);
        alert('Bid placed successfully!');
    });

    async function showItemDetails(itemId) {
        const item = await contract.get_item(itemId);
        itemDetails.innerHTML = `<strong>Owner:</strong> ${item.owner}<br>
                                 <strong>Description:</strong> ${item.description}<br>
                                 <strong>Current Bid:</strong> ${item.current_bid}<br>
                                 <strong>Highest Bidder:</strong> ${item.highest_bidder || 'None'}`;
    }

    const showItemForm = document.getElementById('show-item-form');
    
    showItemForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const itemId = parseInt(document.getElementById('item-id').value, 10);
        await showItemDetails(itemId);
    });
});
