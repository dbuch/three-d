use crate::context::consts;
use crate::core::*;

///
/// A buffer containing per instance data.
/// To send this data to a shader, use the [Program::use_instance_attribute] method.
///
pub struct InstanceBuffer<T: BufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    attribute_count: u32,
    _dummy: T,
}

impl<T: BufferDataType> InstanceBuffer<T> {
    ///
    /// Creates a new empty instance buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            attribute_count: 0,
            _dummy: T::default(),
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data.
    ///
    pub fn new_with_data(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data.
    ///
    pub fn fill(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            if self.attribute_count > 0 {
                consts::DYNAMIC_DRAW
            } else {
                consts::STATIC_DRAW
            },
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.attribute_count = data.len() as u32;
    }

    ///
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_dynamic](InstanceBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_static(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_dynamic](InstanceBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_static(&mut self, data: &[T]) {
        self.fill(data)
    }

    ///
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_static](InstanceBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_dynamic(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_static](InstanceBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_dynamic(&mut self, data: &[T]) {
        self.fill(data)
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> u32 {
        self.attribute_count * T::size()
    }

    ///
    /// The number of instance attributes in the buffer.
    ///
    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl<T: BufferDataType> Drop for InstanceBuffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
