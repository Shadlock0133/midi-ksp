/// Main kRPC service, used by clients to interact with basic server
/// functionality.
mod KRPC {
    /// A server side expression.
    struct Expression;

    /// A server side expression.
    struct Type;

    /// The game scene. See M:KRPC.CurrentGameScene.
    enum GameScene {
        /// The game scene showing the Kerbal Space Center buildings.
        SpaceCenter = 0,
        /// The game scene showing a vessel in flight (or on the
        /// launchpad/runway).
        Flight = 1,
        /// The tracking station.
        TrackingStation = 2,
        /// The Vehicle Assembly Building.
        EditorVAB = 3,
        /// The Space Plane Hangar.
        EditorSPH = 4,
    }

    /// Returns the identifier for the current client.
    /// real name: GetClientID
    fn GetClientID() -> Bytes;

    /// Returns the name of the current client.
    /// This is an empty string if the client has no name.
    /// real name: GetClientName
    fn GetClientName() -> String;

    /// Returns some information about the server, such as the version.
    /// real name: GetStatus
    fn GetStatus() -> Status;

    /// Returns information on all services, procedures, classes, properties
    /// etc. provided by the server.
    /// Can be used by client libraries to automatically create functionality
    /// such as stubs.
    /// real name: GetServices
    fn GetServices() -> Services;

    /// Add a streaming request and return its identifier.
    /// real name: AddStream
    fn AddStream(call: ProcedureCall, start: Bool) -> Stream;

    /// Start a previously added streaming request.
    /// real name: StartStream
    fn StartStream(id: Uint64);

    /// Set the update rate for a stream in Hz.
    /// real name: SetStreamRate
    fn SetStreamRate(id: Uint64, rate: Float);

    /// Remove a streaming request.
    /// real name: RemoveStream
    fn RemoveStream(id: Uint64);

    /// Create an event from a server side expression.
    /// real name: AddEvent
    fn AddEvent(expression: Class) -> Event;

    /// A list of RPC clients that are currently connected to the server.
    /// Each entry in the list is a clients identifier, name and address.
    /// real name: get_Clients
    fn get_Clients() -> List<(Bytes, String, String,)>;

    /// Get the current game scene.
    /// real name: get_CurrentGameScene
    fn get_CurrentGameScene() -> Enumeration;

    /// Whether the game is paused.
    /// real name: get_Paused
    fn get_Paused() -> Bool;

    /// Whether the game is paused.
    /// real name: set_Paused
    fn set_Paused(value: Bool);

    impl Expression {
        /// A constant value of double precision floating point type.
        /// 
        /// <param name="value"></param>
        /// real name: Expression_static_ConstantDouble
        fn ConstantDouble(value: Double) -> Class;

        /// A constant value of single precision floating point type.
        /// 
        /// <param name="value"></param>
        /// real name: Expression_static_ConstantFloat
        fn ConstantFloat(value: Float) -> Class;

        /// A constant value of integer type.
        /// 
        /// <param name="value"></param>
        /// real name: Expression_static_ConstantInt
        fn ConstantInt(value: Sint32) -> Class;

        /// A constant value of boolean type.
        /// 
        /// <param name="value"></param>
        /// real name: Expression_static_ConstantBool
        fn ConstantBool(value: Bool) -> Class;

        /// A constant value of string type.
        /// 
        /// <param name="value"></param>
        /// real name: Expression_static_ConstantString
        fn ConstantString(value: String) -> Class;

        /// An RPC call.
        /// 
        /// <param name="call"></param>
        /// real name: Expression_static_Call
        fn Call(call: ProcedureCall) -> Class;

        /// Equality comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Equal
        fn Equal(arg0: Class, arg1: Class) -> Class;

        /// Inequality comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_NotEqual
        fn NotEqual(arg0: Class, arg1: Class) -> Class;

        /// Greater than numerical comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_GreaterThan
        fn GreaterThan(arg0: Class, arg1: Class) -> Class;

        /// Greater than or equal numerical comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_GreaterThanOrEqual
        fn GreaterThanOrEqual(arg0: Class, arg1: Class) -> Class;

        /// Less than numerical comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_LessThan
        fn LessThan(arg0: Class, arg1: Class) -> Class;

        /// Less than or equal numerical comparison.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_LessThanOrEqual
        fn LessThanOrEqual(arg0: Class, arg1: Class) -> Class;

        /// Boolean and operator.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_And
        fn And(arg0: Class, arg1: Class) -> Class;

        /// Boolean or operator.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Or
        fn Or(arg0: Class, arg1: Class) -> Class;

        /// Boolean exclusive-or operator.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_ExclusiveOr
        fn ExclusiveOr(arg0: Class, arg1: Class) -> Class;

        /// Boolean negation operator.
        /// 
        /// <param name="arg"></param>
        /// real name: Expression_static_Not
        fn Not(arg: Class) -> Class;

        /// Numerical addition.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Add
        fn Add(arg0: Class, arg1: Class) -> Class;

        /// Numerical subtraction.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Subtract
        fn Subtract(arg0: Class, arg1: Class) -> Class;

        /// Numerical multiplication.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Multiply
        fn Multiply(arg0: Class, arg1: Class) -> Class;

        /// Numerical division.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_Divide
        fn Divide(arg0: Class, arg1: Class) -> Class;

        /// Numerical modulo operator.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// # Returns
        /// 
        /// The remainder of arg0 divided by arg1
        /// real name: Expression_static_Modulo
        fn Modulo(arg0: Class, arg1: Class) -> Class;

        /// Numerical power operator.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// # Returns
        /// 
        /// arg0 raised to the power of arg1, with type of arg0
        /// real name: Expression_static_Power
        fn Power(arg0: Class, arg1: Class) -> Class;

        /// Bitwise left shift.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_LeftShift
        fn LeftShift(arg0: Class, arg1: Class) -> Class;

        /// Bitwise right shift.
        /// 
        /// <param name="arg0"></param>
        /// <param name="arg1"></param>
        /// real name: Expression_static_RightShift
        fn RightShift(arg0: Class, arg1: Class) -> Class;

        /// Perform a cast to the given type.
        /// 
        /// <param name="arg"></param>
        /// <param name="type">Type to cast the argument to.</param>
        /// real name: Expression_static_Cast
        fn Cast(arg: Class, r#type: Class) -> Class;

        /// A named parameter of type double.
        /// 
        /// # Returns
        /// 
        /// A named parameter.
        /// 
        /// <param name="name">The name of the parameter.</param>
        /// <param name="type">The type of the parameter.</param>
        /// real name: Expression_static_Parameter
        fn Parameter(name: String, r#type: Class) -> Class;

        /// A function.
        /// 
        /// # Returns
        /// 
        /// A function.
        /// 
        /// <param name="parameters">The parameters of the function.</param>
        /// <param name="body">The body of the function.</param>
        /// real name: Expression_static_Function
        fn Function(parameters: List<Class>, body: Class) -> Class;

        /// A function call.
        /// 
        /// # Returns
        /// 
        /// A function call.
        /// 
        /// <param name="function">The function to call.</param>
        /// <param name="args">The arguments to call the function with.</param>
        /// real name: Expression_static_Invoke
        fn Invoke(function: Class, args: Dictionary<>) -> Class;

        /// Construct a tuple.
        /// 
        /// # Returns
        /// 
        /// The tuple.
        /// 
        /// <param name="elements">The elements.</param>
        /// real name: Expression_static_CreateTuple
        fn CreateTuple(elements: List<Class>) -> Class;

        /// Construct a list.
        /// 
        /// # Returns
        /// 
        /// The list.
        /// 
        /// <param name="values">The value. Should all be of the same
        /// type.</param>
        /// real name: Expression_static_CreateList
        fn CreateList(values: List<Class>) -> Class;

        /// Construct a set.
        /// 
        /// # Returns
        /// 
        /// The set.
        /// 
        /// <param name="values">The values. Should all be of the same
        /// type.</param>
        /// real name: Expression_static_CreateSet
        fn CreateSet(values: Set<>) -> Class;

        /// Construct a dictionary, from a list of corresponding keys and
        /// values.
        /// 
        /// # Returns
        /// 
        /// The dictionary.
        /// 
        /// <param name="keys">The keys. Should all be of the same type.</param>
        /// <param name="values">The values. Should all be of the same
        /// type.</param>
        /// real name: Expression_static_CreateDictionary
        fn CreateDictionary(keys: List<Class>, values: List<Class>) -> Class;

        /// Convert a collection to a list.
        /// 
        /// # Returns
        /// 
        /// The collection as a list.
        /// 
        /// <param name="arg">The collection.</param>
        /// real name: Expression_static_ToList
        fn ToList(arg: Class) -> Class;

        /// Convert a collection to a set.
        /// 
        /// # Returns
        /// 
        /// The collection as a set.
        /// 
        /// <param name="arg">The collection.</param>
        /// real name: Expression_static_ToSet
        fn ToSet(arg: Class) -> Class;

        /// Access an element in a tuple, list or dictionary.
        /// 
        /// # Returns
        /// 
        /// The element.
        /// 
        /// <param name="arg">The tuple, list or dictionary.</param>
        /// <param name="index">The index of the element to access.
        /// A zero indexed integer for a tuple or list, or a key for a
        /// dictionary.</param>
        /// real name: Expression_static_Get
        fn Get(arg: Class, index: Class) -> Class;

        /// Number of elements in a collection.
        /// 
        /// # Returns
        /// 
        /// The number of elements in the collection.
        /// 
        /// <param name="arg">The list, set or dictionary.</param>
        /// real name: Expression_static_Count
        fn Count(arg: Class) -> Class;

        /// Sum all elements of a collection.
        /// 
        /// # Returns
        /// 
        /// The sum of the elements in the collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// real name: Expression_static_Sum
        fn Sum(arg: Class) -> Class;

        /// Maximum of all elements in a collection.
        /// 
        /// # Returns
        /// 
        /// The maximum elements in the collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// real name: Expression_static_Max
        fn Max(arg: Class) -> Class;

        /// Minimum of all elements in a collection.
        /// 
        /// # Returns
        /// 
        /// The minimum elements in the collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// real name: Expression_static_Min
        fn Min(arg: Class) -> Class;

        /// Minimum of all elements in a collection.
        /// 
        /// # Returns
        /// 
        /// The minimum elements in the collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// real name: Expression_static_Average
        fn Average(arg: Class) -> Class;

        /// Run a function on every element in the collection.
        /// 
        /// # Returns
        /// 
        /// The modified collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// <param name="func">The function.</param>
        /// real name: Expression_static_Select
        fn Select(arg: Class, func: Class) -> Class;

        /// Run a function on every element in the collection.
        /// 
        /// # Returns
        /// 
        /// The modified collection.
        /// 
        /// <param name="arg">The list or set.</param>
        /// <param name="func">The function.</param>
        /// real name: Expression_static_Where
        fn Where(arg: Class, func: Class) -> Class;

        /// Determine if a collection contains a value.
        /// 
        /// # Returns
        /// 
        /// Whether the collection contains a value.
        /// 
        /// <param name="arg">The collection.</param>
        /// <param name="value">The value to look for.</param>
        /// real name: Expression_static_Contains
        fn Contains(arg: Class, value: Class) -> Class;

        /// Applies an accumulator function over a sequence.
        /// 
        /// # Returns
        /// 
        /// The accumulated value.
        /// 
        /// <param name="arg">The collection.</param>
        /// <param name="func">The accumulator function.</param>
        /// real name: Expression_static_Aggregate
        fn Aggregate(arg: Class, func: Class) -> Class;

        /// Applies an accumulator function over a sequence, with a given seed.
        /// 
        /// # Returns
        /// 
        /// The accumulated value.
        /// 
        /// <param name="arg">The collection.</param>
        /// <param name="seed">The seed value.</param>
        /// <param name="func">The accumulator function.</param>
        /// real name: Expression_static_AggregateWithSeed
        fn AggregateWithSeed(arg: Class, seed: Class, func: Class) -> Class;

        /// Concatenate two sequences.
        /// 
        /// # Returns
        /// 
        /// The first sequence followed by the second sequence.
        /// 
        /// <param name="arg1">The first sequence.</param>
        /// <param name="arg2">The second sequence.</param>
        /// real name: Expression_static_Concat
        fn Concat(arg1: Class, arg2: Class) -> Class;

        /// Order a collection using a key function.
        /// 
        /// # Returns
        /// 
        /// The ordered collection.
        /// 
        /// <param name="arg">The collection to order.</param>
        /// <param name="key">A function that takes a value from the collection
        /// and generates a key to sort on.</param>
        /// real name: Expression_static_OrderBy
        fn OrderBy(arg: Class, key: Class) -> Class;

        /// Determine whether all items in a collection satisfy a boolean
        /// predicate.
        /// 
        /// # Returns
        /// 
        /// Whether all items satisfy the predicate.
        /// 
        /// <param name="arg">The collection.</param>
        /// <param name="predicate">The predicate function.</param>
        /// real name: Expression_static_All
        fn All(arg: Class, predicate: Class) -> Class;

        /// Determine whether any item in a collection satisfies a boolean
        /// predicate.
        /// 
        /// # Returns
        /// 
        /// Whether any item satisfies the predicate.
        /// 
        /// <param name="arg">The collection.</param>
        /// <param name="predicate">The predicate function.</param>
        /// real name: Expression_static_Any
        fn Any(arg: Class, predicate: Class) -> Class;

    }

    impl Type {
        /// Double type.
        /// real name: Type_static_Double
        fn Double() -> Class;

        /// Float type.
        /// real name: Type_static_Float
        fn Float() -> Class;

        /// Int type.
        /// real name: Type_static_Int
        fn Int() -> Class;

        /// Bool type.
        /// real name: Type_static_Bool
        fn Bool() -> Class;

        /// String type.
        /// real name: Type_static_String
        fn String() -> Class;

    }

}

/// Provides functionality for drawing objects in the flight scene.
/// 
/// # Remarks
/// 
/// For drawing and interacting with the user interface, see the UI service.
mod Drawing {
    /// A line. Created using M:Drawing.AddLine.
    struct Line;

    /// A polygon. Created using M:Drawing.AddPolygon.
    struct Polygon;

    /// Text. Created using M:Drawing.AddText.
    struct Text;

    /// Draw a line in the scene.
    /// 
    /// <param name="start">Position of the start of the line.</param>
    /// <param name="end">Position of the end of the line.</param>
    /// <param name="referenceFrame">Reference frame that the positions are
    /// in.</param>
    /// <param name="visible">Whether the line is visible.</param>
    /// real name: AddLine
    fn AddLine(start: (Double, Double, Double,), end: (Double, Double, Double,), referenceFrame: Class, visible: Bool) -> Class;

    /// Draw a direction vector in the scene, from the center of mass of the
    /// active vessel.
    /// 
    /// <param name="direction">Direction to draw the line in.</param>
    /// <param name="referenceFrame">Reference frame that the direction is
    /// in.</param>
    /// <param name="length">The length of the line.</param>
    /// <param name="visible">Whether the line is visible.</param>
    /// real name: AddDirection
    fn AddDirection(direction: (Double, Double, Double,), referenceFrame: Class, length: Float, visible: Bool) -> Class;

    /// Draw a polygon in the scene, defined by a list of vertices.
    /// 
    /// <param name="vertices">Vertices of the polygon.</param>
    /// <param name="referenceFrame">Reference frame that the vertices are
    /// in.</param>
    /// <param name="visible">Whether the polygon is visible.</param>
    /// real name: AddPolygon
    fn AddPolygon(vertices: List<(Double, Double, Double,)>, referenceFrame: Class, visible: Bool) -> Class;

    /// Draw text in the scene.
    /// 
    /// <param name="text">The string to draw.</param>
    /// <param name="referenceFrame">Reference frame that the text position is
    /// in.</param>
    /// <param name="position">Position of the text.</param>
    /// <param name="rotation">Rotation of the text, as a quaternion.</param>
    /// <param name="visible">Whether the text is visible.</param>
    /// real name: AddText
    fn AddText(text: String, referenceFrame: Class, position: (Double, Double, Double,), rotation: (Double, Double, Double, Double,), visible: Bool) -> Class;

    /// Remove all objects being drawn.
    /// 
    /// <param name="clientOnly">If true, only remove objects created by the
    /// calling client.</param>
    /// real name: Clear
    fn Clear(clientOnly: Bool);

    impl Line {
        /// Remove the object.
        /// real name: Line_Remove
        fn Remove(&self);

        /// Start position of the line.
        /// real name: Line_get_Start
        fn get_Start(&self) -> (Double, Double, Double,);

        /// Start position of the line.
        /// real name: Line_set_Start
        fn set_Start(&self, value: (Double, Double, Double,));

        /// End position of the line.
        /// real name: Line_get_End
        fn get_End(&self) -> (Double, Double, Double,);

        /// End position of the line.
        /// real name: Line_set_End
        fn set_End(&self, value: (Double, Double, Double,));

        /// Set the color
        /// real name: Line_get_Color
        fn get_Color(&self) -> (Double, Double, Double,);

        /// Set the color
        /// real name: Line_set_Color
        fn set_Color(&self, value: (Double, Double, Double,));

        /// Set the thickness
        /// real name: Line_get_Thickness
        fn get_Thickness(&self) -> Float;

        /// Set the thickness
        /// real name: Line_set_Thickness
        fn set_Thickness(&self, value: Float);

        /// Reference frame for the positions of the object.
        /// real name: Line_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// Reference frame for the positions of the object.
        /// real name: Line_set_ReferenceFrame
        fn set_ReferenceFrame(&self, value: Class);

        /// Whether the object is visible.
        /// real name: Line_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the object is visible.
        /// real name: Line_set_Visible
        fn set_Visible(&self, value: Bool);

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Line_get_Material
        fn get_Material(&self) -> String;

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Line_set_Material
        fn set_Material(&self, value: String);

    }

    impl Polygon {
        /// Remove the object.
        /// real name: Polygon_Remove
        fn Remove(&self);

        /// Vertices for the polygon.
        /// real name: Polygon_get_Vertices
        fn get_Vertices(&self) -> List<(Double, Double, Double,)>;

        /// Vertices for the polygon.
        /// real name: Polygon_set_Vertices
        fn set_Vertices(&self, value: List<(Double, Double, Double,)>);

        /// Set the color
        /// real name: Polygon_get_Color
        fn get_Color(&self) -> (Double, Double, Double,);

        /// Set the color
        /// real name: Polygon_set_Color
        fn set_Color(&self, value: (Double, Double, Double,));

        /// Set the thickness
        /// real name: Polygon_get_Thickness
        fn get_Thickness(&self) -> Float;

        /// Set the thickness
        /// real name: Polygon_set_Thickness
        fn set_Thickness(&self, value: Float);

        /// Reference frame for the positions of the object.
        /// real name: Polygon_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// Reference frame for the positions of the object.
        /// real name: Polygon_set_ReferenceFrame
        fn set_ReferenceFrame(&self, value: Class);

        /// Whether the object is visible.
        /// real name: Polygon_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the object is visible.
        /// real name: Polygon_set_Visible
        fn set_Visible(&self, value: Bool);

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Polygon_get_Material
        fn get_Material(&self) -> String;

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Polygon_set_Material
        fn set_Material(&self, value: String);

    }

    impl Text {
        /// A list of all available fonts.
        /// real name: Text_static_AvailableFonts
        fn AvailableFonts() -> List<String>;

        /// Remove the object.
        /// real name: Text_Remove
        fn Remove(&self);

        /// Position of the text.
        /// real name: Text_get_Position
        fn get_Position(&self) -> (Double, Double, Double,);

        /// Position of the text.
        /// real name: Text_set_Position
        fn set_Position(&self, value: (Double, Double, Double,));

        /// Rotation of the text as a quaternion.
        /// real name: Text_get_Rotation
        fn get_Rotation(&self) -> (Double, Double, Double, Double,);

        /// Rotation of the text as a quaternion.
        /// real name: Text_set_Rotation
        fn set_Rotation(&self, value: (Double, Double, Double, Double,));

        /// The text string
        /// real name: Text_get_Content
        fn get_Content(&self) -> String;

        /// The text string
        /// real name: Text_set_Content
        fn set_Content(&self, value: String);

        /// Name of the font
        /// real name: Text_get_Font
        fn get_Font(&self) -> String;

        /// Name of the font
        /// real name: Text_set_Font
        fn set_Font(&self, value: String);

        /// Font size.
        /// real name: Text_get_Size
        fn get_Size(&self) -> Sint32;

        /// Font size.
        /// real name: Text_set_Size
        fn set_Size(&self, value: Sint32);

        /// Character size.
        /// real name: Text_get_CharacterSize
        fn get_CharacterSize(&self) -> Float;

        /// Character size.
        /// real name: Text_set_CharacterSize
        fn set_CharacterSize(&self, value: Float);

        /// Font style.
        /// real name: Text_get_Style
        fn get_Style(&self) -> Enumeration;

        /// Font style.
        /// real name: Text_set_Style
        fn set_Style(&self, value: Enumeration);

        /// Alignment.
        /// real name: Text_get_Alignment
        fn get_Alignment(&self) -> Enumeration;

        /// Alignment.
        /// real name: Text_set_Alignment
        fn set_Alignment(&self, value: Enumeration);

        /// Line spacing.
        /// real name: Text_get_LineSpacing
        fn get_LineSpacing(&self) -> Float;

        /// Line spacing.
        /// real name: Text_set_LineSpacing
        fn set_LineSpacing(&self, value: Float);

        /// Anchor.
        /// real name: Text_get_Anchor
        fn get_Anchor(&self) -> Enumeration;

        /// Anchor.
        /// real name: Text_set_Anchor
        fn set_Anchor(&self, value: Enumeration);

        /// Set the color
        /// real name: Text_get_Color
        fn get_Color(&self) -> (Double, Double, Double,);

        /// Set the color
        /// real name: Text_set_Color
        fn set_Color(&self, value: (Double, Double, Double,));

        /// Reference frame for the positions of the object.
        /// real name: Text_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// Reference frame for the positions of the object.
        /// real name: Text_set_ReferenceFrame
        fn set_ReferenceFrame(&self, value: Class);

        /// Whether the object is visible.
        /// real name: Text_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the object is visible.
        /// real name: Text_set_Visible
        fn set_Visible(&self, value: Bool);

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Text_get_Material
        fn get_Material(&self) -> String;

        /// Material used to render the object.
        /// Creates the material from a shader with the given name.
        /// real name: Text_set_Material
        fn set_Material(&self, value: String);

    }

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/104535-112-magic-smoke-industries-infernal-robotics-202/">Infernal Robotics</a>.
mod InfernalRobotics {
    /// Represents a servo. Obtained using
    /// M:InfernalRobotics.ServoGroup.Servos,
    /// M:InfernalRobotics.ServoGroup.ServoWithName
    /// or M:InfernalRobotics.ServoWithName.
    struct Servo;

    /// A group of servos, obtained by calling M:InfernalRobotics.ServoGroups
    /// or M:InfernalRobotics.ServoGroupWithName. Represents the "Servo Groups"
    /// in the InfernalRobotics UI.
    struct ServoGroup;

    /// A list of all the servo groups in the given <paramref name="vessel.
    /// real name: ServoGroups
    fn ServoGroups(vessel: Class) -> List<Class>;

    /// Returns the servo group in the given <paramref name="vessel with the
    /// given <paramref name="name,
    /// or `null` if none exists. If multiple servo groups have the same name,
    /// only one of them is returned.
    /// 
    /// <param name="vessel">Vessel to check.</param>
    /// <param name="name">Name of servo group to find.</param>
    /// real name: ServoGroupWithName
    fn ServoGroupWithName(vessel: Class, name: String) -> Option<Class>;

    /// Returns the servo in the given <paramref name="vessel with the given
    /// <paramref name="name or
    /// `null` if none exists. If multiple servos have the same name, only one
    /// of them is returned.
    /// 
    /// <param name="vessel">Vessel to check.</param>
    /// <param name="name">Name of the servo to find.</param>
    /// real name: ServoWithName
    fn ServoWithName(vessel: Class, name: String) -> Option<Class>;

    /// Whether Infernal Robotics is installed.
    /// real name: get_Available
    fn get_Available() -> Bool;

    /// Whether Infernal Robotics API is ready.
    /// real name: get_Ready
    fn get_Ready() -> Bool;

    impl Servo {
        /// Moves the servo to the right.
        /// real name: Servo_MoveRight
        fn MoveRight(&self);

        /// Moves the servo to the left.
        /// real name: Servo_MoveLeft
        fn MoveLeft(&self);

        /// Moves the servo to the center.
        /// real name: Servo_MoveCenter
        fn MoveCenter(&self);

        /// Moves the servo to the next preset.
        /// real name: Servo_MoveNextPreset
        fn MoveNextPreset(&self);

        /// Moves the servo to the previous preset.
        /// real name: Servo_MovePrevPreset
        fn MovePrevPreset(&self);

        /// Moves the servo to <paramref name="position and sets the
        /// speed multiplier to <paramref name="speed.
        /// 
        /// <param name="position">The position to move the servo to.</param>
        /// <param name="speed">Speed multiplier for the movement.</param>
        /// real name: Servo_MoveTo
        fn MoveTo(&self, position: Float, speed: Float);

        /// Stops the servo.
        /// real name: Servo_Stop
        fn Stop(&self);

        /// The name of the servo.
        /// real name: Servo_get_Name
        fn get_Name(&self) -> String;

        /// The name of the servo.
        /// real name: Servo_set_Name
        fn set_Name(&self, value: String);

        /// The part containing the servo.
        /// real name: Servo_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the servo should be highlighted in-game.
        /// real name: Servo_set_Highlight
        fn set_Highlight(&self, value: Bool);

        /// The position of the servo.
        /// real name: Servo_get_Position
        fn get_Position(&self) -> Float;

        /// The minimum position of the servo, specified by the part
        /// configuration.
        /// real name: Servo_get_MinConfigPosition
        fn get_MinConfigPosition(&self) -> Float;

        /// The maximum position of the servo, specified by the part
        /// configuration.
        /// real name: Servo_get_MaxConfigPosition
        fn get_MaxConfigPosition(&self) -> Float;

        /// The minimum position of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_get_MinPosition
        fn get_MinPosition(&self) -> Float;

        /// The minimum position of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_set_MinPosition
        fn set_MinPosition(&self, value: Float);

        /// The maximum position of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_get_MaxPosition
        fn get_MaxPosition(&self) -> Float;

        /// The maximum position of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_set_MaxPosition
        fn set_MaxPosition(&self, value: Float);

        /// The speed multiplier of the servo, specified by the part
        /// configuration.
        /// real name: Servo_get_ConfigSpeed
        fn get_ConfigSpeed(&self) -> Float;

        /// The speed multiplier of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_get_Speed
        fn get_Speed(&self) -> Float;

        /// The speed multiplier of the servo, specified by the in-game tweak
        /// menu.
        /// real name: Servo_set_Speed
        fn set_Speed(&self, value: Float);

        /// The current speed at which the servo is moving.
        /// real name: Servo_get_CurrentSpeed
        fn get_CurrentSpeed(&self) -> Float;

        /// The current speed at which the servo is moving.
        /// real name: Servo_set_CurrentSpeed
        fn set_CurrentSpeed(&self, value: Float);

        /// The current speed multiplier set in the UI.
        /// real name: Servo_get_Acceleration
        fn get_Acceleration(&self) -> Float;

        /// The current speed multiplier set in the UI.
        /// real name: Servo_set_Acceleration
        fn set_Acceleration(&self, value: Float);

        /// Whether the servo is moving.
        /// real name: Servo_get_IsMoving
        fn get_IsMoving(&self) -> Bool;

        /// Whether the servo is freely moving.
        /// real name: Servo_get_IsFreeMoving
        fn get_IsFreeMoving(&self) -> Bool;

        /// Whether the servo is locked.
        /// real name: Servo_get_IsLocked
        fn get_IsLocked(&self) -> Bool;

        /// Whether the servo is locked.
        /// real name: Servo_set_IsLocked
        fn set_IsLocked(&self, value: Bool);

        /// Whether the servos axis is inverted.
        /// real name: Servo_get_IsAxisInverted
        fn get_IsAxisInverted(&self) -> Bool;

        /// Whether the servos axis is inverted.
        /// real name: Servo_set_IsAxisInverted
        fn set_IsAxisInverted(&self, value: Bool);

    }

    impl ServoGroup {
        /// Returns the servo with the given <paramref name="name from this
        /// group,
        /// or `null` if none exists.
        /// 
        /// <param name="name">Name of servo to find.</param>
        /// real name: ServoGroup_ServoWithName
        fn ServoWithName(&self, name: String) -> Option<Class>;

        /// Moves all of the servos in the group to the right.
        /// real name: ServoGroup_MoveRight
        fn MoveRight(&self);

        /// Moves all of the servos in the group to the left.
        /// real name: ServoGroup_MoveLeft
        fn MoveLeft(&self);

        /// Moves all of the servos in the group to the center.
        /// real name: ServoGroup_MoveCenter
        fn MoveCenter(&self);

        /// Moves all of the servos in the group to the next preset.
        /// real name: ServoGroup_MoveNextPreset
        fn MoveNextPreset(&self);

        /// Moves all of the servos in the group to the previous preset.
        /// real name: ServoGroup_MovePrevPreset
        fn MovePrevPreset(&self);

        /// Stops the servos in the group.
        /// real name: ServoGroup_Stop
        fn Stop(&self);

        /// The name of the group.
        /// real name: ServoGroup_get_Name
        fn get_Name(&self) -> String;

        /// The name of the group.
        /// real name: ServoGroup_set_Name
        fn set_Name(&self, value: String);

        /// The key assigned to be the "forward" key for the group.
        /// real name: ServoGroup_get_ForwardKey
        fn get_ForwardKey(&self) -> String;

        /// The key assigned to be the "forward" key for the group.
        /// real name: ServoGroup_set_ForwardKey
        fn set_ForwardKey(&self, value: String);

        /// The key assigned to be the "reverse" key for the group.
        /// real name: ServoGroup_get_ReverseKey
        fn get_ReverseKey(&self) -> String;

        /// The key assigned to be the "reverse" key for the group.
        /// real name: ServoGroup_set_ReverseKey
        fn set_ReverseKey(&self, value: String);

        /// The speed multiplier for the group.
        /// real name: ServoGroup_get_Speed
        fn get_Speed(&self) -> Float;

        /// The speed multiplier for the group.
        /// real name: ServoGroup_set_Speed
        fn set_Speed(&self, value: Float);

        /// Whether the group is expanded in the InfernalRobotics UI.
        /// real name: ServoGroup_get_Expanded
        fn get_Expanded(&self) -> Bool;

        /// Whether the group is expanded in the InfernalRobotics UI.
        /// real name: ServoGroup_set_Expanded
        fn set_Expanded(&self, value: Bool);

        /// The servos that are in the group.
        /// real name: ServoGroup_get_Servos
        fn get_Servos(&self) -> List<Class>;

        /// The parts containing the servos in the group.
        /// real name: ServoGroup_get_Parts
        fn get_Parts(&self) -> List<Class>;

    }

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/22809-13x-kerbal-alarm-clock-v3850-may-30/">Kerbal Alarm Clock</a>.
mod KerbalAlarmClock {
    /// Represents an alarm. Obtained by calling
    /// M:KerbalAlarmClock.Alarms,
    /// M:KerbalAlarmClock.AlarmWithName or
    /// M:KerbalAlarmClock.AlarmsWithType.
    struct Alarm;

    /// The action performed by an alarm when it fires.
    enum AlarmAction {
        /// Don't do anything at all...
        DoNothing = 0,
        /// Don't do anything, and delete the alarm.
        DoNothingDeleteWhenPassed = 1,
        /// Drop out of time warp.
        KillWarp = 2,
        /// Drop out of time warp.
        KillWarpOnly = 3,
        /// Display a message.
        MessageOnly = 4,
        /// Pause the game.
        PauseGame = 5,
    }

    /// The type of an alarm.
    enum AlarmType {
        /// An alarm for a specific date/time or a specific period in the
        /// future.
        Raw = 0,
        /// An alarm based on the next maneuver node on the current ships
        /// flight path.
        /// This node will be stored and can be restored when you come back to
        /// the ship.
        Maneuver = 1,
        /// See M:KerbalAlarmClock.AlarmType.Maneuver.
        ManeuverAuto = 2,
        /// An alarm for furthest part of the orbit from the planet.
        Apoapsis = 3,
        /// An alarm for nearest part of the orbit from the planet.
        Periapsis = 4,
        /// Ascending node for the targeted object, or equatorial ascending
        /// node.
        AscendingNode = 5,
        /// Descending node for the targeted object, or equatorial descending
        /// node.
        DescendingNode = 6,
        /// An alarm based on the closest approach of this vessel to the
        /// targeted
        /// vessel, some number of orbits into the future.
        Closest = 7,
        /// An alarm based on the expiry or deadline of contracts in career
        /// modes.
        Contract = 8,
        /// See M:KerbalAlarmClock.AlarmType.Contract.
        ContractAuto = 9,
        /// An alarm that is attached to a crew member.
        Crew = 10,
        /// An alarm that is triggered when a selected target comes within a
        /// chosen distance.
        Distance = 11,
        /// An alarm based on the time in the "Earth" alternative Universe (aka
        /// the Real World).
        EarthTime = 12,
        /// An alarm that fires as your landed craft passes under the orbit of
        /// your target.
        LaunchRendevous = 13,
        /// An alarm manually based on when the next SOI point is on the flight
        /// path
        /// or set to continually monitor the active flight path and add alarms
        /// as it
        /// detects SOI changes.
        SOIChange = 14,
        /// See M:KerbalAlarmClock.AlarmType.SOIChange.
        SOIChangeAuto = 15,
        /// An alarm based on Interplanetary Transfer Phase Angles, i.e. when
        /// should
        /// I launch to planet X? Based on Kosmo Not's post and used in Olex's
        /// Calculator.
        Transfer = 16,
        /// See M:KerbalAlarmClock.AlarmType.Transfer.
        TransferModelled = 17,
    }

    /// Get the alarm with the given <paramref name="name, or `null`
    /// if no alarms have that name. If more than one alarm has the name,
    /// only returns one of them.
    /// 
    /// <param name="name">Name of the alarm to search for.</param>
    /// real name: AlarmWithName
    fn AlarmWithName(name: String) -> Option<Class>;

    /// Get a list of alarms of the specified <paramref name="type.
    /// 
    /// <param name="type">Type of alarm to return.</param>
    /// real name: AlarmsWithType
    fn AlarmsWithType(r#type: Enumeration) -> List<Class>;

    /// Create a new alarm and return it.
    /// 
    /// <param name="type">Type of the new alarm.</param>
    /// <param name="name">Name of the new alarm.</param>
    /// <param name="ut">Time at which the new alarm should trigger.</param>
    /// real name: CreateAlarm
    fn CreateAlarm(r#type: Enumeration, name: String, ut: Double) -> Class;

    /// Whether Kerbal Alarm Clock is available.
    /// real name: get_Available
    fn get_Available() -> Bool;

    /// A list of all the alarms.
    /// real name: get_Alarms
    fn get_Alarms() -> List<Class>;

    impl Alarm {
        /// Removes the alarm.
        /// real name: Alarm_Remove
        fn Remove(&self);

        /// The action that the alarm triggers.
        /// real name: Alarm_get_Action
        fn get_Action(&self) -> Enumeration;

        /// The action that the alarm triggers.
        /// real name: Alarm_set_Action
        fn set_Action(&self, value: Enumeration);

        /// The number of seconds before the event that the alarm will fire.
        /// real name: Alarm_get_Margin
        fn get_Margin(&self) -> Double;

        /// The number of seconds before the event that the alarm will fire.
        /// real name: Alarm_set_Margin
        fn set_Margin(&self, value: Double);

        /// The time at which the alarm will fire.
        /// real name: Alarm_get_Time
        fn get_Time(&self) -> Double;

        /// The time at which the alarm will fire.
        /// real name: Alarm_set_Time
        fn set_Time(&self, value: Double);

        /// The type of the alarm.
        /// real name: Alarm_get_Type
        fn get_Type(&self) -> Enumeration;

        /// The unique identifier for the alarm.
        /// real name: Alarm_get_ID
        fn get_ID(&self) -> String;

        /// The short name of the alarm.
        /// real name: Alarm_get_Name
        fn get_Name(&self) -> String;

        /// The short name of the alarm.
        /// real name: Alarm_set_Name
        fn set_Name(&self, value: String);

        /// The long description of the alarm.
        /// real name: Alarm_get_Notes
        fn get_Notes(&self) -> String;

        /// The long description of the alarm.
        /// real name: Alarm_set_Notes
        fn set_Notes(&self, value: String);

        /// The number of seconds until the alarm will fire.
        /// real name: Alarm_get_Remaining
        fn get_Remaining(&self) -> Double;

        /// Whether the alarm will be repeated after it has fired.
        /// real name: Alarm_get_Repeat
        fn get_Repeat(&self) -> Bool;

        /// Whether the alarm will be repeated after it has fired.
        /// real name: Alarm_set_Repeat
        fn set_Repeat(&self, value: Bool);

        /// The time delay to automatically create an alarm after it has fired.
        /// real name: Alarm_get_RepeatPeriod
        fn get_RepeatPeriod(&self) -> Double;

        /// The time delay to automatically create an alarm after it has fired.
        /// real name: Alarm_set_RepeatPeriod
        fn set_RepeatPeriod(&self, value: Double);

        /// The vessel that the alarm is attached to.
        /// real name: Alarm_get_Vessel
        fn get_Vessel(&self) -> Class;

        /// The vessel that the alarm is attached to.
        /// real name: Alarm_set_Vessel
        fn set_Vessel(&self, value: Class);

        /// The celestial body the vessel is departing from.
        /// real name: Alarm_get_XferOriginBody
        fn get_XferOriginBody(&self) -> Class;

        /// The celestial body the vessel is departing from.
        /// real name: Alarm_set_XferOriginBody
        fn set_XferOriginBody(&self, value: Class);

        /// The celestial body the vessel is arriving at.
        /// real name: Alarm_get_XferTargetBody
        fn get_XferTargetBody(&self) -> Class;

        /// The celestial body the vessel is arriving at.
        /// real name: Alarm_set_XferTargetBody
        fn set_XferTargetBody(&self, value: Class);

    }

}

/// This service provides functionality to interact with
/// <a
/// href="https://forum.kerbalspaceprogram.com/index.php?/topic/139167-13-remotetech-v188-2017-09-03/">RemoteTech</a>.
mod RemoteTech {
    /// A RemoteTech antenna. Obtained by calling M:RemoteTech.Comms.Antennas
    /// or M:RemoteTech.Antenna.
    struct Antenna;

    /// Communications for a vessel.
    struct Comms;

    /// The type of object an antenna is targetting.
    /// See M:RemoteTech.Antenna.Target.
    enum Target {
        /// The active vessel.
        ActiveVessel = 0,
        /// A celestial body.
        CelestialBody = 1,
        /// A ground station.
        GroundStation = 2,
        /// A specific vessel.
        Vessel = 3,
        /// No target.
        None = 4,
    }

    /// Get a communications object, representing the communication capability
    /// of a particular vessel.
    /// real name: Comms
    fn Comms(vessel: Class) -> Class;

    /// Get the antenna object for a particular part.
    /// real name: Antenna
    fn Antenna(part: Class) -> Class;

    /// Whether RemoteTech is installed.
    /// real name: get_Available
    fn get_Available() -> Bool;

    /// The names of the ground stations.
    /// real name: get_GroundStations
    fn get_GroundStations() -> List<String>;

    impl Antenna {
        /// Get the part containing this antenna.
        /// real name: Antenna_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the antenna has a connection.
        /// real name: Antenna_get_HasConnection
        fn get_HasConnection(&self) -> Bool;

        /// The object that the antenna is targetting.
        /// This property can be used to set the target to
        /// M:RemoteTech.Target.None or M:RemoteTech.Target.ActiveVessel.
        /// To set the target to a celestial body, ground station or vessel see
        /// M:RemoteTech.Antenna.TargetBody,
        /// M:RemoteTech.Antenna.TargetGroundStation and
        /// M:RemoteTech.Antenna.TargetVessel.
        /// real name: Antenna_get_Target
        fn get_Target(&self) -> Enumeration;

        /// The object that the antenna is targetting.
        /// This property can be used to set the target to
        /// M:RemoteTech.Target.None or M:RemoteTech.Target.ActiveVessel.
        /// To set the target to a celestial body, ground station or vessel see
        /// M:RemoteTech.Antenna.TargetBody,
        /// M:RemoteTech.Antenna.TargetGroundStation and
        /// M:RemoteTech.Antenna.TargetVessel.
        /// real name: Antenna_set_Target
        fn set_Target(&self, value: Enumeration);

        /// The celestial body the antenna is targetting.
        /// real name: Antenna_get_TargetBody
        fn get_TargetBody(&self) -> Class;

        /// The celestial body the antenna is targetting.
        /// real name: Antenna_set_TargetBody
        fn set_TargetBody(&self, value: Class);

        /// The ground station the antenna is targetting.
        /// real name: Antenna_get_TargetGroundStation
        fn get_TargetGroundStation(&self) -> String;

        /// The ground station the antenna is targetting.
        /// real name: Antenna_set_TargetGroundStation
        fn set_TargetGroundStation(&self, value: String);

        /// The vessel the antenna is targetting.
        /// real name: Antenna_get_TargetVessel
        fn get_TargetVessel(&self) -> Class;

        /// The vessel the antenna is targetting.
        /// real name: Antenna_set_TargetVessel
        fn set_TargetVessel(&self, value: Class);

    }

    impl Comms {
        /// The signal delay between the this vessel and another vessel, in
        /// seconds.
        /// 
        /// <param name="other"></param>
        /// real name: Comms_SignalDelayToVessel
        fn SignalDelayToVessel(&self, other: Class) -> Double;

        /// Get the vessel.
        /// real name: Comms_get_Vessel
        fn get_Vessel(&self) -> Class;

        /// Whether the vessel can be controlled locally.
        /// real name: Comms_get_HasLocalControl
        fn get_HasLocalControl(&self) -> Bool;

        /// Whether the vessel has a flight computer on board.
        /// real name: Comms_get_HasFlightComputer
        fn get_HasFlightComputer(&self) -> Bool;

        /// Whether the vessel has any connection.
        /// real name: Comms_get_HasConnection
        fn get_HasConnection(&self) -> Bool;

        /// Whether the vessel has a connection to a ground station.
        /// real name: Comms_get_HasConnectionToGroundStation
        fn get_HasConnectionToGroundStation(&self) -> Bool;

        /// The shortest signal delay to the vessel, in seconds.
        /// real name: Comms_get_SignalDelay
        fn get_SignalDelay(&self) -> Double;

        /// The signal delay between the vessel and the closest ground station,
        /// in seconds.
        /// real name: Comms_get_SignalDelayToGroundStation
        fn get_SignalDelayToGroundStation(&self) -> Double;

        /// The antennas for this vessel.
        /// real name: Comms_get_Antennas
        fn get_Antennas(&self) -> List<Class>;

    }

}

/// Provides functionality to interact with Kerbal Space Program. This includes
/// controlling
/// the active vessel, managing its resources, planning maneuver nodes and
/// auto-piloting.
mod SpaceCenter {
    /// Provides basic auto-piloting utilities for a vessel.
    /// Created by calling M:SpaceCenter.Vessel.AutoPilot.
    /// 
    /// # Remarks
    /// 
    /// If a client engages the auto-pilot and then closes its connection to
    /// the server,
    /// the auto-pilot will be disengaged and its target reference frame,
    /// direction and roll
    /// reset to default.
    struct AutoPilot;

    /// Controls the game's camera.
    /// Obtained by calling M:SpaceCenter.Camera.
    struct Camera;

    /// Represents a celestial body (such as a planet or moon).
    /// See M:SpaceCenter.Bodies.
    struct CelestialBody;

    /// Represents a communication node in the network. For example, a vessel
    /// or the KSC.
    struct CommLink;

    /// Represents a communication node in the network. For example, a vessel
    /// or the KSC.
    struct CommNode;

    /// Used to interact with CommNet for a given vessel.
    /// Obtained by calling M:SpaceCenter.Vessel.Comms.
    struct Comms;

    /// A contract. Can be accessed using M:SpaceCenter.ContractManager.
    struct Contract;

    /// Contracts manager.
    /// Obtained by calling M:SpaceCenter.ContractManager.
    struct ContractManager;

    /// A contract parameter. See M:SpaceCenter.Contract.Parameters.
    struct ContractParameter;

    /// Used to manipulate the controls of a vessel. This includes adjusting the
    /// throttle, enabling/disabling systems such as SAS and RCS, or altering
    /// the
    /// direction in which the vessel is pointing.
    /// Obtained by calling M:SpaceCenter.Vessel.Control.
    /// 
    /// # Remarks
    /// 
    /// Control inputs (such as pitch, yaw and roll) are zeroed when all clients
    /// that have set one or more of these inputs are no longer connected.
    struct Control;

    /// Represents crew in a vessel. Can be obtained using
    /// M:SpaceCenter.Vessel.Crew.
    struct CrewMember;

    /// Used to get flight telemetry for a vessel, by calling
    /// M:SpaceCenter.Vessel.Flight.
    /// All of the information returned by this class is given in the reference
    /// frame
    /// passed to that method.
    /// Obtained by calling M:SpaceCenter.Vessel.Flight.
    /// 
    /// # Remarks
    /// 
    /// To get orbital information, such as the apoapsis or inclination, see
    /// T:SpaceCenter.Orbit.
    struct Flight;

    /// Represents a maneuver node. Can be created using
    /// M:SpaceCenter.Control.AddNode.
    struct Node;

    /// Describes an orbit. For example, the orbit of a vessel, obtained by
    /// calling
    /// M:SpaceCenter.Vessel.Orbit, or a celestial body, obtained by calling
    /// M:SpaceCenter.CelestialBody.Orbit.
    struct Orbit;

    /// An antenna. Obtained by calling M:SpaceCenter.Part.Antenna.
    struct Antenna;

    /// A cargo bay. Obtained by calling M:SpaceCenter.Part.CargoBay.
    struct CargoBay;

    /// An aerodynamic control surface. Obtained by calling
    /// M:SpaceCenter.Part.ControlSurface.
    struct ControlSurface;

    /// A decoupler. Obtained by calling M:SpaceCenter.Part.Decoupler
    struct Decoupler;

    /// A docking port. Obtained by calling M:SpaceCenter.Part.DockingPort
    struct DockingPort;

    /// An engine, including ones of various types.
    /// For example liquid fuelled gimballed engines, solid rocket boosters and
    /// jet engines.
    /// Obtained by calling M:SpaceCenter.Part.Engine.
    /// 
    /// # Remarks
    /// 
    /// For RCS thrusters M:SpaceCenter.Part.RCS.
    struct Engine;

    /// Obtained by calling M:SpaceCenter.Part.Experiment.
    struct Experiment;

    /// A fairing. Obtained by calling M:SpaceCenter.Part.Fairing.
    struct Fairing;

    /// Obtained by calling M:SpaceCenter.Part.AddForce.
    struct Force;

    /// An air intake. Obtained by calling M:SpaceCenter.Part.Intake.
    struct Intake;

    /// A launch clamp. Obtained by calling M:SpaceCenter.Part.LaunchClamp.
    struct LaunchClamp;

    /// A landing leg. Obtained by calling M:SpaceCenter.Part.Leg.
    struct Leg;

    /// A light. Obtained by calling M:SpaceCenter.Part.Light.
    struct Light;

    /// This can be used to interact with a specific part module. This includes
    /// part modules in
    /// stock KSP, and those added by mods.
    /// 
    /// In KSP, each part has zero or more
    /// <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/CFG_File_Documentation#MODULES">PartModules</a>
    /// associated with it. Each one contains some of the functionality of the
    /// part.
    /// For example, an engine has a "ModuleEngines" part module that contains
    /// all the
    /// functionality of an engine.
    struct Module;

    /// A parachute. Obtained by calling M:SpaceCenter.Part.Parachute.
    struct Parachute;

    /// Represents an individual part. Vessels are made up of multiple parts.
    /// Instances of this class can be obtained by several methods in
    /// T:SpaceCenter.Parts.
    struct Part;

    /// Instances of this class are used to interact with the parts of a vessel.
    /// An instance can be obtained by calling M:SpaceCenter.Vessel.Parts.
    struct Parts;

    /// A propellant for an engine. Obtains by calling
    /// M:SpaceCenter.Engine.Propellants.
    struct Propellant;

    /// An RCS block or thruster. Obtained by calling M:SpaceCenter.Part.RCS.
    struct RCS;

    /// A radiator. Obtained by calling M:SpaceCenter.Part.Radiator.
    struct Radiator;

    /// A reaction wheel. Obtained by calling M:SpaceCenter.Part.ReactionWheel.
    struct ReactionWheel;

    /// A resource converter. Obtained by calling
    /// M:SpaceCenter.Part.ResourceConverter.
    struct ResourceConverter;

    /// A resource harvester (drill). Obtained by calling
    /// M:SpaceCenter.Part.ResourceHarvester.
    struct ResourceHarvester;

    /// Obtained by calling M:SpaceCenter.Experiment.Data.
    struct ScienceData;

    /// Obtained by calling M:SpaceCenter.Experiment.ScienceSubject.
    struct ScienceSubject;

    /// A sensor, such as a thermometer. Obtained by calling
    /// M:SpaceCenter.Part.Sensor.
    struct Sensor;

    /// A solar panel. Obtained by calling M:SpaceCenter.Part.SolarPanel.
    struct SolarPanel;

    /// The component of an T:SpaceCenter.Engine or T:SpaceCenter.RCS part that
    /// generates thrust.
    /// Can obtained by calling M:SpaceCenter.Engine.Thrusters or
    /// M:SpaceCenter.RCS.Thrusters.
    /// 
    /// # Remarks
    /// 
    /// Engines can consist of multiple thrusters.
    /// For example, the S3 KS-25x4 "Mammoth" has four rocket nozzels, and so
    /// consists of
    /// four thrusters.
    struct Thruster;

    /// A wheel. Includes landing gear and rover wheels.
    /// Obtained by calling M:SpaceCenter.Part.Wheel.
    /// Can be used to control the motors, steering and deployment of wheels,
    /// among other things.
    struct Wheel;

    /// Represents a reference frame for positions, rotations and
    /// velocities. Contains:
    /// - The position of the origin.
    /// - The directions of the x, y and z axes.
    /// - The linear velocity of the frame.
    /// - The angular velocity of the frame.
    /// 
    /// # Remarks
    /// 
    /// This class does not contain any properties or methods. It is only
    /// used as a parameter to other functions.
    struct ReferenceFrame;

    /// An individual resource stored within a part.
    /// Created using methods in the T:SpaceCenter.Resources class.
    struct Resource;

    /// Transfer resources between parts.
    struct ResourceTransfer;

    /// Represents the collection of resources stored in a vessel, stage or
    /// part.
    /// Created by calling M:SpaceCenter.Vessel.Resources,
    /// M:SpaceCenter.Vessel.ResourcesInDecoupleStage or
    /// M:SpaceCenter.Part.Resources.
    struct Resources;

    /// These objects are used to interact with vessels in KSP. This includes
    /// getting
    /// orbital and flight data, manipulating control inputs and managing
    /// resources.
    /// Created using M:SpaceCenter.ActiveVessel or M:SpaceCenter.Vessels.
    struct Vessel;

    /// Represents a waypoint. Can be created using
    /// M:SpaceCenter.WaypointManager.AddWaypoint.
    struct Waypoint;

    /// Waypoints are the location markers you can see on the map view showing
    /// you where contracts are targeted for.
    /// With this structure, you can obtain coordinate data for the locations
    /// of these waypoints.
    /// Obtained by calling M:SpaceCenter.WaypointManager.
    struct WaypointManager;

    /// See M:SpaceCenter.Camera.Mode.
    enum CameraMode {
        /// The camera is showing the active vessel, in "auto" mode.
        Automatic = 0,
        /// The camera is showing the active vessel, in "free" mode.
        Free = 1,
        /// The camera is showing the active vessel, in "chase" mode.
        Chase = 2,
        /// The camera is showing the active vessel, in "locked" mode.
        Locked = 3,
        /// The camera is showing the active vessel, in "orbital" mode.
        Orbital = 4,
        /// The Intra-Vehicular Activity view is being shown.
        IVA = 5,
        /// The map view is being shown.
        Map = 6,
    }

    /// The type of a communication link.
    /// See M:SpaceCenter.CommLink.Type.
    enum CommLinkType {
        /// Link is to a base station on Kerbin.
        Home = 0,
        /// Link is to a control source, for example a manned spacecraft.
        Control = 1,
        /// Link is to a relay satellite.
        Relay = 2,
    }

    /// The state of a contract. See M:SpaceCenter.Contract.State.
    enum ContractState {
        /// The contract is active.
        Active = 0,
        /// The contract has been canceled.
        Canceled = 1,
        /// The contract has been completed.
        Completed = 2,
        /// The deadline for the contract has expired.
        DeadlineExpired = 3,
        /// The contract has been declined.
        Declined = 4,
        /// The contract has been failed.
        Failed = 5,
        /// The contract has been generated.
        Generated = 6,
        /// The contract has been offered to the player.
        Offered = 7,
        /// The contract was offered to the player, but the offer expired.
        OfferExpired = 8,
        /// The contract has been withdrawn.
        Withdrawn = 9,
    }

    /// See M:SpaceCenter.Control.InputMode.
    enum ControlInputMode {
        /// Control inputs are added to the vessels current control inputs.
        Additive = 0,
        /// Control inputs (when they are non-zero) override the vessels
        /// current control inputs.
        Override = 1,
    }

    /// The control source of a vessel.
    /// See M:SpaceCenter.Control.Source.
    enum ControlSource {
        /// Vessel is controlled by a Kerbal.
        Kerbal = 0,
        /// Vessel is controlled by a probe core.
        Probe = 1,
        /// Vessel is not controlled.
        None = 2,
    }

    /// The control state of a vessel.
    /// See M:SpaceCenter.Control.State.
    enum ControlState {
        /// Full controllable.
        Full = 0,
        /// Partially controllable.
        Partial = 1,
        /// Not controllable.
        None = 2,
    }

    /// The type of a crew member.
    /// See M:SpaceCenter.CrewMember.Type.
    enum CrewMemberType {
        /// An applicant for crew.
        Applicant = 0,
        /// Rocket crew.
        Crew = 1,
        /// A tourist.
        Tourist = 2,
        /// An unowned crew member.
        Unowned = 3,
    }

    /// The game mode.
    /// Returned by T:SpaceCenter.GameMode
    enum GameMode {
        /// Sandbox mode.
        Sandbox = 0,
        /// Career mode.
        Career = 1,
        /// Science career mode.
        Science = 2,
        /// Science sandbox mode.
        ScienceSandbox = 3,
        /// Mission mode.
        Mission = 4,
        /// Mission builder mode.
        MissionBuilder = 5,
        /// Scenario mode.
        Scenario = 6,
        /// Scenario mode that cannot be resumed.
        ScenarioNonResumable = 7,
    }

    /// The state of an antenna. See M:SpaceCenter.Antenna.State.
    enum AntennaState {
        /// Antenna is fully deployed.
        Deployed = 0,
        /// Antenna is fully retracted.
        Retracted = 1,
        /// Antenna is being deployed.
        Deploying = 2,
        /// Antenna is being retracted.
        Retracting = 3,
        /// Antenna is broken.
        Broken = 4,
    }

    /// The state of a cargo bay. See M:SpaceCenter.CargoBay.State.
    enum CargoBayState {
        /// Cargo bay is fully open.
        Open = 0,
        /// Cargo bay closed and locked.
        Closed = 1,
        /// Cargo bay is opening.
        Opening = 2,
        /// Cargo bay is closing.
        Closing = 3,
    }

    /// The state of a docking port. See M:SpaceCenter.DockingPort.State.
    enum DockingPortState {
        /// The docking port is ready to dock to another docking port.
        Ready = 0,
        /// The docking port is docked to another docking port, or docked to
        /// another part (from the VAB/SPH).
        Docked = 1,
        /// The docking port is very close to another docking port,
        /// but has not docked. It is using magnetic force to acquire a solid
        /// dock.
        Docking = 2,
        /// The docking port has just been undocked from another docking port,
        /// and is disabled until it moves away by a sufficient distance
        /// (M:SpaceCenter.DockingPort.ReengageDistance).
        Undocking = 3,
        /// The docking port has a shield, and the shield is closed.
        Shielded = 4,
        /// The docking ports shield is currently opening/closing.
        Moving = 5,
    }

    /// The state of a landing leg. See M:SpaceCenter.Leg.State.
    enum LegState {
        /// Landing leg is fully deployed.
        Deployed = 0,
        /// Landing leg is fully retracted.
        Retracted = 1,
        /// Landing leg is being deployed.
        Deploying = 2,
        /// Landing leg is being retracted.
        Retracting = 3,
        /// Landing leg is broken.
        Broken = 4,
    }

    /// The state of the motor on a powered wheel. See
    /// M:SpaceCenter.Wheel.MotorState.
    enum MotorState {
        /// The motor is idle.
        Idle = 0,
        /// The motor is running.
        Running = 1,
        /// The motor is disabled.
        Disabled = 2,
        /// The motor is inoperable.
        Inoperable = 3,
        /// The motor does not have enough resources to run.
        NotEnoughResources = 4,
    }

    /// The state of a parachute. See M:SpaceCenter.Parachute.State.
    enum ParachuteState {
        /// The parachute is safely tucked away inside its housing.
        Stowed = 0,
        /// The parachute is armed for deployment. (RealChutes only)
        Armed = 1,
        /// The parachute is still stowed, but ready to semi-deploy.
        /// (Stock parachutes only)
        Active = 2,
        /// The parachute has been deployed and is providing some drag,
        /// but is not fully deployed yet. (Stock parachutes only)
        SemiDeployed = 3,
        /// The parachute is fully deployed.
        Deployed = 4,
        /// The parachute has been cut.
        Cut = 5,
    }

    /// The state of a radiator. T:SpaceCenter.RadiatorState
    enum RadiatorState {
        /// Radiator is fully extended.
        Extended = 0,
        /// Radiator is fully retracted.
        Retracted = 1,
        /// Radiator is being extended.
        Extending = 2,
        /// Radiator is being retracted.
        Retracting = 3,
        /// Radiator is being broken.
        Broken = 4,
    }

    /// The state of a resource converter. See
    /// M:SpaceCenter.ResourceConverter.State.
    enum ResourceConverterState {
        /// Converter is running.
        Running = 0,
        /// Converter is idle.
        Idle = 1,
        /// Converter is missing a required resource.
        MissingResource = 2,
        /// No available storage for output resource.
        StorageFull = 3,
        /// At preset resource capacity.
        Capacity = 4,
        /// Unknown state. Possible with modified resource converters.
        /// In this case, check M:SpaceCenter.ResourceConverter.StatusInfo for
        /// more information.
        Unknown = 5,
    }

    /// The state of a resource harvester. See
    /// M:SpaceCenter.ResourceHarvester.State.
    enum ResourceHarvesterState {
        /// The drill is deploying.
        Deploying = 0,
        /// The drill is deployed and ready.
        Deployed = 1,
        /// The drill is retracting.
        Retracting = 2,
        /// The drill is retracted.
        Retracted = 3,
        /// The drill is running.
        Active = 4,
    }

    /// The state of a solar panel. See M:SpaceCenter.SolarPanel.State.
    enum SolarPanelState {
        /// Solar panel is fully extended.
        Extended = 0,
        /// Solar panel is fully retracted.
        Retracted = 1,
        /// Solar panel is being extended.
        Extending = 2,
        /// Solar panel is being retracted.
        Retracting = 3,
        /// Solar panel is broken.
        Broken = 4,
    }

    /// The state of a wheel. See M:SpaceCenter.Wheel.State.
    enum WheelState {
        /// Wheel is fully deployed.
        Deployed = 0,
        /// Wheel is fully retracted.
        Retracted = 1,
        /// Wheel is being deployed.
        Deploying = 2,
        /// Wheel is being retracted.
        Retracting = 3,
        /// Wheel is broken.
        Broken = 4,
    }

    /// The way in which a resource flows between parts. See
    /// M:SpaceCenter.Resources.FlowMode.
    enum ResourceFlowMode {
        /// The resource flows to any part in the vessel. For example, electric
        /// charge.
        Vessel = 0,
        /// The resource flows from parts in the first stage, followed by the
        /// second,
        /// and so on. For example, mono-propellant.
        Stage = 1,
        /// The resource flows between adjacent parts within the vessel. For
        /// example,
        /// liquid fuel or oxidizer.
        Adjacent = 2,
        /// The resource does not flow. For example, solid fuel.
        None = 3,
    }

    /// The behavior of the SAS auto-pilot. See M:SpaceCenter.AutoPilot.SASMode.
    enum SASMode {
        /// Stability assist mode. Dampen out any rotation.
        StabilityAssist = 0,
        /// Point in the burn direction of the next maneuver node.
        Maneuver = 1,
        /// Point in the prograde direction.
        Prograde = 2,
        /// Point in the retrograde direction.
        Retrograde = 3,
        /// Point in the orbit normal direction.
        Normal = 4,
        /// Point in the orbit anti-normal direction.
        AntiNormal = 5,
        /// Point in the orbit radial direction.
        Radial = 6,
        /// Point in the orbit anti-radial direction.
        AntiRadial = 7,
        /// Point in the direction of the current target.
        Target = 8,
        /// Point away from the current target.
        AntiTarget = 9,
    }

    /// The mode of the speed reported in the navball.
    /// See M:SpaceCenter.Control.SpeedMode.
    enum SpeedMode {
        /// Speed is relative to the vessel's orbit.
        Orbit = 0,
        /// Speed is relative to the surface of the body being orbited.
        Surface = 1,
        /// Speed is relative to the current target.
        Target = 2,
    }

    /// The situation a vessel is in.
    /// See M:SpaceCenter.Vessel.Situation.
    enum VesselSituation {
        /// Vessel is awaiting launch.
        PreLaunch = 0,
        /// Vessel is orbiting a body.
        Orbiting = 1,
        /// Vessel is on a sub-orbital trajectory.
        SubOrbital = 2,
        /// Escaping.
        Escaping = 3,
        /// Vessel is flying through an atmosphere.
        Flying = 4,
        /// Vessel is landed on the surface of a body.
        Landed = 5,
        /// Vessel has splashed down in an ocean.
        Splashed = 6,
        /// Vessel is docked to another.
        Docked = 7,
    }

    /// The type of a vessel.
    /// See M:SpaceCenter.Vessel.Type.
    enum VesselType {
        /// Base.
        Base = 0,
        /// Debris.
        Debris = 1,
        /// Lander.
        Lander = 2,
        /// Plane.
        Plane = 3,
        /// Probe.
        Probe = 4,
        /// Relay.
        Relay = 5,
        /// Rover.
        Rover = 6,
        /// Ship.
        Ship = 7,
        /// Station.
        Station = 8,
    }

    /// The time warp mode.
    /// Returned by T:SpaceCenter.WarpMode
    enum WarpMode {
        /// Time warp is active, and in regular "on-rails" mode.
        Rails = 0,
        /// Time warp is active, and in physical time warp mode.
        Physics = 1,
        /// Time warp is not active.
        None = 2,
    }

    /// Clears the current target.
    /// real name: ClearTarget
    fn ClearTarget();

    /// Returns a list of vessels from the given <paramref name="craftDirectory
    /// that can be launched.
    /// 
    /// <param name="craftDirectory">Name of the directory in the current saves
    /// "Ships" directory. For example `"VAB"` or `"SPH"`.</param>
    /// real name: LaunchableVessels
    fn LaunchableVessels(craftDirectory: String) -> List<String>;

    /// Launch a vessel.
    /// 
    /// <param name="craftDirectory">Name of the directory in the current saves
    /// "Ships" directory, that contains the craft file.
    /// For example `"VAB"` or `"SPH"`.</param>
    /// <param name="name">Name of the vessel to launch. This is the name of
    /// the ".craft" file
    /// in the save directory, without the ".craft" file extension.</param>
    /// <param name="launchSite">Name of the launch site. For example
    /// `"LaunchPad"` or
    /// `"Runway"`.</param>
    /// <param name="recover">If true and there is a vessel on the launch site,
    /// recover it before launching.</param>
    /// # Remarks
    /// 
    /// Throws an exception if any of the games pre-flight checks fail.
    /// real name: LaunchVessel
    fn LaunchVessel(craftDirectory: String, name: String, launchSite: String, recover: Bool);

    /// Launch a new vessel from the VAB onto the launchpad.
    /// 
    /// <param name="name">Name of the vessel to launch.</param>
    /// <param name="recover">If true and there is a vessel on the launch pad,
    /// recover it before launching.</param>
    /// # Remarks
    /// 
    /// This is equivalent to calling M:SpaceCenter.LaunchVessel with the craft
    /// directory
    /// set to "VAB" and the launch site set to "LaunchPad".
    /// Throws an exception if any of the games pre-flight checks fail.
    /// real name: LaunchVesselFromVAB
    fn LaunchVesselFromVAB(name: String, recover: Bool);

    /// Launch a new vessel from the SPH onto the runway.
    /// 
    /// <param name="name">Name of the vessel to launch.</param>
    /// <param name="recover">If true and there is a vessel on the runway,
    /// recover it before launching.</param>
    /// # Remarks
    /// 
    /// This is equivalent to calling M:SpaceCenter.LaunchVessel with the craft
    /// directory
    /// set to "SPH" and the launch site set to "Runway".
    /// Throws an exception if any of the games pre-flight checks fail.
    /// real name: LaunchVesselFromSPH
    fn LaunchVesselFromSPH(name: String, recover: Bool);

    /// Save the game with a given name.
    /// This will create a save file called `name.sfs` in the folder of the
    /// current save game.
    /// real name: Save
    fn Save(name: String);

    /// Load the game with the given name.
    /// This will create a load a save file called `name.sfs` from the folder
    /// of the
    /// current save game.
    /// real name: Load
    fn Load(name: String);

    /// Save a quicksave.
    /// 
    /// # Remarks
    /// 
    /// This is the same as calling M:SpaceCenter.Save with the name
    /// "quicksave".
    /// real name: Quicksave
    fn Quicksave();

    /// Load a quicksave.
    /// 
    /// # Remarks
    /// 
    /// This is the same as calling M:SpaceCenter.Load with the name
    /// "quicksave".
    /// real name: Quickload
    fn Quickload();

    /// Returns `true` if regular "on-rails" time warp can be used, at the
    /// specified warp
    /// <paramref name="factor. The maximum time warp rate is limited by
    /// various things,
    /// including how close the active vessel is to a planet. See
    /// <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP
    /// wiki</a>
    /// for details.
    /// 
    /// <param name="factor">The warp factor to check.</param>
    /// real name: CanRailsWarpAt
    fn CanRailsWarpAt(factor: Sint32) -> Bool;

    /// Uses time acceleration to warp forward to a time in the future,
    /// specified
    /// by universal time <paramref name="ut. This call blocks until the desired
    /// time is reached. Uses regular "on-rails" or physical time warp as
    /// appropriate.
    /// For example, physical time warp is used when the active vessel is
    /// traveling
    /// through an atmosphere. When using regular "on-rails" time warp, the warp
    /// rate is limited by <paramref name="maxRailsRate, and when using physical
    /// time warp, the warp rate is limited by <paramref name="maxPhysicsRate.
    /// 
    /// <param name="ut">The universal time to warp to, in seconds.</param>
    /// <param name="maxRailsRate">The maximum warp rate in regular "on-rails"
    /// time warp.
    /// </param>
    /// <param name="maxPhysicsRate">The maximum warp rate in physical time
    /// warp.</param>
    /// # Returns
    /// 
    /// When the time warp is complete.
    /// real name: WarpTo
    fn WarpTo(ut: Double, maxRailsRate: Float, maxPhysicsRate: Float);

    /// Converts a position from one reference frame to another.
    /// 
    /// <param name="position">Position, as a vector, in reference frame
    /// <paramref name="from.</param>
    /// <param name="from">The reference frame that the position is in.</param>
    /// <param name="to">The reference frame to covert the position to.</param>
    /// # Returns
    /// 
    /// The corresponding position, as a vector, in reference frame
    /// <paramref name="to.
    /// real name: TransformPosition
    fn TransformPosition(position: (Double, Double, Double,), from: Class, to: Class) -> (Double, Double, Double,);

    /// Converts a direction from one reference frame to another.
    /// 
    /// <param name="direction">Direction, as a vector, in reference frame
    /// <paramref name="from. </param>
    /// <param name="from">The reference frame that the direction is in.</param>
    /// <param name="to">The reference frame to covert the direction to.</param>
    /// # Returns
    /// 
    /// The corresponding direction, as a vector, in reference frame
    /// <paramref name="to.
    /// real name: TransformDirection
    fn TransformDirection(direction: (Double, Double, Double,), from: Class, to: Class) -> (Double, Double, Double,);

    /// Converts a rotation from one reference frame to another.
    /// 
    /// <param name="rotation">Rotation, as a quaternion of the form <math>(x,
    /// y, z, w)</math>,
    /// in reference frame <paramref name="from.</param>
    /// <param name="from">The reference frame that the rotation is in.</param>
    /// <param name="to">The reference frame to covert the rotation to.</param>
    /// # Returns
    /// 
    /// The corresponding rotation, as a quaternion of the form
    /// <math>(x, y, z, w)</math>, in reference frame <paramref name="to.
    /// real name: TransformRotation
    fn TransformRotation(rotation: (Double, Double, Double, Double,), from: Class, to: Class) -> (Double, Double, Double, Double,);

    /// Converts a velocity (acting at the specified position) from one
    /// reference frame
    /// to another. The position is required to take the relative angular
    /// velocity of the
    /// reference frames into account.
    /// 
    /// <param name="position">Position, as a vector, in reference frame
    /// <paramref name="from.</param>
    /// <param name="velocity">Velocity, as a vector that points in the
    /// direction of travel and
    /// whose magnitude is the speed in meters per second, in reference frame
    /// <paramref name="from.</param>
    /// <param name="from">The reference frame that the position and velocity
    /// are in.</param>
    /// <param name="to">The reference frame to covert the velocity to.</param>
    /// # Returns
    /// 
    /// The corresponding velocity, as a vector, in reference frame
    /// <paramref name="to.
    /// real name: TransformVelocity
    fn TransformVelocity(position: (Double, Double, Double,), velocity: (Double, Double, Double,), from: Class, to: Class) -> (Double, Double, Double,);

    /// Cast a ray from a given position in a given direction, and return the
    /// distance to the hit point.
    /// If no hit occurs, returns infinity.
    /// 
    /// <param name="position">Position, as a vector, of the origin of the
    /// ray.</param>
    /// <param name="direction">Direction of the ray, as a unit vector.</param>
    /// <param name="referenceFrame">The reference frame that the position and
    /// direction are in.</param>
    /// # Returns
    /// 
    /// The distance to the hit, in meters, or infinity if there was no hit.
    /// real name: RaycastDistance
    fn RaycastDistance(position: (Double, Double, Double,), direction: (Double, Double, Double,), referenceFrame: Class) -> Double;

    /// Cast a ray from a given position in a given direction, and return the
    /// part that it hits.
    /// If no hit occurs, returns `null`.
    /// 
    /// <param name="position">Position, as a vector, of the origin of the
    /// ray.</param>
    /// <param name="direction">Direction of the ray, as a unit vector.</param>
    /// <param name="referenceFrame">The reference frame that the position and
    /// direction are in.</param>
    /// # Returns
    /// 
    /// The part that was hit or `null` if there was no hit.
    /// real name: RaycastPart
    fn RaycastPart(position: (Double, Double, Double,), direction: (Double, Double, Double,), referenceFrame: Class) -> Option<Class>;

    /// The current mode the game is in.
    /// real name: get_GameMode
    fn get_GameMode() -> Enumeration;

    /// The current amount of science.
    /// real name: get_Science
    fn get_Science() -> Float;

    /// The current amount of funds.
    /// real name: get_Funds
    fn get_Funds() -> Double;

    /// The current amount of reputation.
    /// real name: get_Reputation
    fn get_Reputation() -> Float;

    /// The currently active vessel.
    /// real name: get_ActiveVessel
    fn get_ActiveVessel() -> Class;

    /// The currently active vessel.
    /// real name: set_ActiveVessel
    fn set_ActiveVessel(value: Class);

    /// A list of all the vessels in the game.
    /// real name: get_Vessels
    fn get_Vessels() -> List<Class>;

    /// A dictionary of all celestial bodies (planets, moons, etc.) in the game,
    /// keyed by the name of the body.
    /// real name: get_Bodies
    fn get_Bodies() -> Dictionary<>;

    /// The currently targeted celestial body.
    /// real name: get_TargetBody
    fn get_TargetBody() -> Option<Class>;

    /// The currently targeted celestial body.
    /// real name: set_TargetBody
    fn set_TargetBody(value: Class);

    /// The currently targeted vessel.
    /// real name: get_TargetVessel
    fn get_TargetVessel() -> Option<Class>;

    /// The currently targeted vessel.
    /// real name: set_TargetVessel
    fn set_TargetVessel(value: Class);

    /// The currently targeted docking port.
    /// real name: get_TargetDockingPort
    fn get_TargetDockingPort() -> Option<Class>;

    /// The currently targeted docking port.
    /// real name: set_TargetDockingPort
    fn set_TargetDockingPort(value: Class);

    /// The waypoint manager.
    /// real name: get_WaypointManager
    fn get_WaypointManager() -> Class;

    /// The contract manager.
    /// real name: get_ContractManager
    fn get_ContractManager() -> Class;

    /// An object that can be used to control the camera.
    /// real name: get_Camera
    fn get_Camera() -> Class;

    /// Whether the UI is visible.
    /// real name: get_UIVisible
    fn get_UIVisible() -> Bool;

    /// Whether the UI is visible.
    /// real name: set_UIVisible
    fn set_UIVisible(value: Bool);

    /// Whether the navball is visible.
    /// real name: get_Navball
    fn get_Navball() -> Bool;

    /// Whether the navball is visible.
    /// real name: set_Navball
    fn set_Navball(value: Bool);

    /// The current universal time in seconds.
    /// real name: get_UT
    fn get_UT() -> Double;

    /// The value of the <a
    /// href="https://en.wikipedia.org/wiki/Gravitational_constant">
    /// gravitational constant</a> G in <math>N(m/kg)^2</math>.
    /// real name: get_G
    fn get_G() -> Double;

    /// The current time warp mode. Returns M:SpaceCenter.WarpMode.None if time
    /// warp is not active, M:SpaceCenter.WarpMode.Rails if regular "on-rails"
    /// time warp
    /// is active, or M:SpaceCenter.WarpMode.Physics if physical time warp is
    /// active.
    /// real name: get_WarpMode
    fn get_WarpMode() -> Enumeration;

    /// The current warp rate. This is the rate at which time is passing for
    /// either on-rails or physical time warp. For example, a value of 10 means
    /// time is passing 10x faster than normal. Returns 1 if time warp is not
    /// active.
    /// real name: get_WarpRate
    fn get_WarpRate() -> Float;

    /// The current warp factor. This is the index of the rate at which time
    /// is passing for either regular "on-rails" or physical time warp. Returns
    /// 0
    /// if time warp is not active. When in on-rails time warp, this is equal to
    /// M:SpaceCenter.RailsWarpFactor, and in physics time warp, this is equal
    /// to
    /// M:SpaceCenter.PhysicsWarpFactor.
    /// real name: get_WarpFactor
    fn get_WarpFactor() -> Float;

    /// The time warp rate, using regular "on-rails" time warp. A value between
    /// 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp
    /// is active.
    /// 
    /// If requested time warp factor cannot be set, it will be set to the next
    /// lowest possible value. For example, if the vessel is too close to a
    /// planet. See <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">
    /// the KSP wiki</a> for details.
    /// real name: get_RailsWarpFactor
    fn get_RailsWarpFactor() -> Sint32;

    /// The time warp rate, using regular "on-rails" time warp. A value between
    /// 0 and 7 inclusive. 0 means no time warp. Returns 0 if physical time warp
    /// is active.
    /// 
    /// If requested time warp factor cannot be set, it will be set to the next
    /// lowest possible value. For example, if the vessel is too close to a
    /// planet. See <a
    /// href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">
    /// the KSP wiki</a> for details.
    /// real name: set_RailsWarpFactor
    fn set_RailsWarpFactor(value: Sint32);

    /// The physical time warp rate. A value between 0 and 3 inclusive. 0 means
    /// no time warp. Returns 0 if regular "on-rails" time warp is active.
    /// real name: get_PhysicsWarpFactor
    fn get_PhysicsWarpFactor() -> Sint32;

    /// The physical time warp rate. A value between 0 and 3 inclusive. 0 means
    /// no time warp. Returns 0 if regular "on-rails" time warp is active.
    /// real name: set_PhysicsWarpFactor
    fn set_PhysicsWarpFactor(value: Sint32);

    /// The current maximum regular "on-rails" warp factor that can be set.
    /// A value between 0 and 7 inclusive. See
    /// <a href="https://wiki.kerbalspaceprogram.com/wiki/Time_warp">the KSP
    /// wiki</a>
    /// for details.
    /// real name: get_MaximumRailsWarpFactor
    fn get_MaximumRailsWarpFactor() -> Sint32;

    /// Whether <a
    /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a> is installed.
    /// real name: get_FARAvailable
    fn get_FARAvailable() -> Bool;

    impl Antenna {
        /// Transmit data.
        /// real name: Antenna_Transmit
        fn Transmit(&self);

        /// Cancel current transmission of data.
        /// real name: Antenna_Cancel
        fn Cancel(&self);

        /// The part object for this antenna.
        /// real name: Antenna_get_Part
        fn get_Part(&self) -> Class;

        /// The current state of the antenna.
        /// real name: Antenna_get_State
        fn get_State(&self) -> Enumeration;

        /// Whether the antenna is deployable.
        /// real name: Antenna_get_Deployable
        fn get_Deployable(&self) -> Bool;

        /// Whether the antenna is deployed.
        /// 
        /// # Remarks
        /// 
        /// Fixed antennas are always deployed.
        /// Returns an error if you try to deploy a fixed antenna.
        /// real name: Antenna_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the antenna is deployed.
        /// 
        /// # Remarks
        /// 
        /// Fixed antennas are always deployed.
        /// Returns an error if you try to deploy a fixed antenna.
        /// real name: Antenna_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// Whether data can be transmitted by this antenna.
        /// real name: Antenna_get_CanTransmit
        fn get_CanTransmit(&self) -> Bool;

        /// Whether partial data transmission is permitted.
        /// real name: Antenna_get_AllowPartial
        fn get_AllowPartial(&self) -> Bool;

        /// Whether partial data transmission is permitted.
        /// real name: Antenna_set_AllowPartial
        fn set_AllowPartial(&self, value: Bool);

        /// The power of the antenna.
        /// real name: Antenna_get_Power
        fn get_Power(&self) -> Double;

        /// Whether the antenna can be combined with other antennae on the
        /// vessel
        /// to boost the power.
        /// real name: Antenna_get_Combinable
        fn get_Combinable(&self) -> Bool;

        /// Exponent used to calculate the combined power of multiple antennae
        /// on a vessel.
        /// real name: Antenna_get_CombinableExponent
        fn get_CombinableExponent(&self) -> Double;

        /// Interval between sending packets in seconds.
        /// real name: Antenna_get_PacketInterval
        fn get_PacketInterval(&self) -> Float;

        /// Amount of data sent per packet in Mits.
        /// real name: Antenna_get_PacketSize
        fn get_PacketSize(&self) -> Float;

        /// Units of electric charge consumed per packet sent.
        /// real name: Antenna_get_PacketResourceCost
        fn get_PacketResourceCost(&self) -> Double;

    }

    impl AutoPilot {
        /// Engage the auto-pilot.
        /// real name: AutoPilot_Engage
        fn Engage(&self);

        /// Disengage the auto-pilot.
        /// real name: AutoPilot_Disengage
        fn Disengage(&self);

        /// Blocks until the vessel is pointing in the target direction and has
        /// the target roll (if set). Throws an exception if the auto-pilot has
        /// not been engaged.
        /// real name: AutoPilot_Wait
        fn Wait(&self);

        /// Set target pitch and heading angles.
        /// 
        /// <param name="pitch">Target pitch angle, in degrees between -90° and
        /// +90°.</param>
        /// <param name="heading">Target heading angle, in degrees between 0°
        /// and 360°.</param>
        /// real name: AutoPilot_TargetPitchAndHeading
        fn TargetPitchAndHeading(&self, pitch: Float, heading: Float);

        /// The error, in degrees, between the direction the ship has been asked
        /// to point in and the direction it is pointing in. Throws an
        /// exception if the auto-pilot
        /// has not been engaged and SAS is not enabled or is in stability
        /// assist mode.
        /// real name: AutoPilot_get_Error
        fn get_Error(&self) -> Float;

        /// The error, in degrees, between the vessels current and target pitch.
        /// Throws an exception if the auto-pilot has not been engaged.
        /// real name: AutoPilot_get_PitchError
        fn get_PitchError(&self) -> Float;

        /// The error, in degrees, between the vessels current and target
        /// heading.
        /// Throws an exception if the auto-pilot has not been engaged.
        /// real name: AutoPilot_get_HeadingError
        fn get_HeadingError(&self) -> Float;

        /// The error, in degrees, between the vessels current and target roll.
        /// Throws an exception if the auto-pilot has not been engaged or no
        /// target roll is set.
        /// real name: AutoPilot_get_RollError
        fn get_RollError(&self) -> Float;

        /// The reference frame for the target direction
        /// (M:SpaceCenter.AutoPilot.TargetDirection).
        /// 
        /// # Remarks
        /// 
        /// An error will be thrown if this property is set to a reference
        /// frame that rotates with
        /// the vessel being controlled, as it is impossible to rotate the
        /// vessel in such a
        /// reference frame.
        /// real name: AutoPilot_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame for the target direction
        /// (M:SpaceCenter.AutoPilot.TargetDirection).
        /// 
        /// # Remarks
        /// 
        /// An error will be thrown if this property is set to a reference
        /// frame that rotates with
        /// the vessel being controlled, as it is impossible to rotate the
        /// vessel in such a
        /// reference frame.
        /// real name: AutoPilot_set_ReferenceFrame
        fn set_ReferenceFrame(&self, value: Class);

        /// The target pitch, in degrees, between -90° and +90°.
        /// real name: AutoPilot_get_TargetPitch
        fn get_TargetPitch(&self) -> Float;

        /// The target pitch, in degrees, between -90° and +90°.
        /// real name: AutoPilot_set_TargetPitch
        fn set_TargetPitch(&self, value: Float);

        /// The target heading, in degrees, between 0° and 360°.
        /// real name: AutoPilot_get_TargetHeading
        fn get_TargetHeading(&self) -> Float;

        /// The target heading, in degrees, between 0° and 360°.
        /// real name: AutoPilot_set_TargetHeading
        fn set_TargetHeading(&self, value: Float);

        /// The target roll, in degrees. `NaN` if no target roll is set.
        /// real name: AutoPilot_get_TargetRoll
        fn get_TargetRoll(&self) -> Float;

        /// The target roll, in degrees. `NaN` if no target roll is set.
        /// real name: AutoPilot_set_TargetRoll
        fn set_TargetRoll(&self, value: Float);

        /// Direction vector corresponding to the target pitch and heading.
        /// This is in the reference frame specified by
        /// T:SpaceCenter.ReferenceFrame.
        /// real name: AutoPilot_get_TargetDirection
        fn get_TargetDirection(&self) -> (Double, Double, Double,);

        /// Direction vector corresponding to the target pitch and heading.
        /// This is in the reference frame specified by
        /// T:SpaceCenter.ReferenceFrame.
        /// real name: AutoPilot_set_TargetDirection
        fn set_TargetDirection(&self, value: (Double, Double, Double,));

        /// The state of SAS.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.Control.SAS
        /// real name: AutoPilot_get_SAS
        fn get_SAS(&self) -> Bool;

        /// The state of SAS.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.Control.SAS
        /// real name: AutoPilot_set_SAS
        fn set_SAS(&self, value: Bool);

        /// The current T:SpaceCenter.SASMode.
        /// These modes are equivalent to the mode buttons to the left of the
        /// navball that appear
        /// when SAS is enabled.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.Control.SASMode
        /// real name: AutoPilot_get_SASMode
        fn get_SASMode(&self) -> Enumeration;

        /// The current T:SpaceCenter.SASMode.
        /// These modes are equivalent to the mode buttons to the left of the
        /// navball that appear
        /// when SAS is enabled.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.Control.SASMode
        /// real name: AutoPilot_set_SASMode
        fn set_SASMode(&self, value: Enumeration);

        /// The threshold at which the autopilot will try to match the target
        /// roll angle, if any.
        /// Defaults to 5 degrees.
        /// real name: AutoPilot_get_RollThreshold
        fn get_RollThreshold(&self) -> Double;

        /// The threshold at which the autopilot will try to match the target
        /// roll angle, if any.
        /// Defaults to 5 degrees.
        /// real name: AutoPilot_set_RollThreshold
        fn set_RollThreshold(&self, value: Double);

        /// The maximum amount of time that the vessel should need to come to a
        /// complete stop.
        /// This determines the maximum angular velocity of the vessel.
        /// A vector of three stopping times, in seconds, one for each of the
        /// pitch, roll
        /// and yaw axes. Defaults to 0.5 seconds for each axis.
        /// real name: AutoPilot_get_StoppingTime
        fn get_StoppingTime(&self) -> (Double, Double, Double,);

        /// The maximum amount of time that the vessel should need to come to a
        /// complete stop.
        /// This determines the maximum angular velocity of the vessel.
        /// A vector of three stopping times, in seconds, one for each of the
        /// pitch, roll
        /// and yaw axes. Defaults to 0.5 seconds for each axis.
        /// real name: AutoPilot_set_StoppingTime
        fn set_StoppingTime(&self, value: (Double, Double, Double,));

        /// The time the vessel should take to come to a stop pointing in the
        /// target direction.
        /// This determines the angular acceleration used to decelerate the
        /// vessel.
        /// A vector of three times, in seconds, one for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 5 seconds for each axis.
        /// real name: AutoPilot_get_DecelerationTime
        fn get_DecelerationTime(&self) -> (Double, Double, Double,);

        /// The time the vessel should take to come to a stop pointing in the
        /// target direction.
        /// This determines the angular acceleration used to decelerate the
        /// vessel.
        /// A vector of three times, in seconds, one for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 5 seconds for each axis.
        /// real name: AutoPilot_set_DecelerationTime
        fn set_DecelerationTime(&self, value: (Double, Double, Double,));

        /// The angle at which the autopilot considers the vessel to be pointing
        /// close to the target.
        /// This determines the midpoint of the target velocity attenuation
        /// function.
        /// A vector of three angles, in degrees, one for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 1° for each axis.
        /// real name: AutoPilot_get_AttenuationAngle
        fn get_AttenuationAngle(&self) -> (Double, Double, Double,);

        /// The angle at which the autopilot considers the vessel to be pointing
        /// close to the target.
        /// This determines the midpoint of the target velocity attenuation
        /// function.
        /// A vector of three angles, in degrees, one for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 1° for each axis.
        /// real name: AutoPilot_set_AttenuationAngle
        fn set_AttenuationAngle(&self, value: (Double, Double, Double,));

        /// Whether the rotation rate controllers PID parameters should be
        /// automatically tuned
        /// using the vessels moment of inertia and available torque. Defaults
        /// to `true`.
        /// See M:SpaceCenter.AutoPilot.TimeToPeak and
        /// M:SpaceCenter.AutoPilot.Overshoot.
        /// real name: AutoPilot_get_AutoTune
        fn get_AutoTune(&self) -> Bool;

        /// Whether the rotation rate controllers PID parameters should be
        /// automatically tuned
        /// using the vessels moment of inertia and available torque. Defaults
        /// to `true`.
        /// See M:SpaceCenter.AutoPilot.TimeToPeak and
        /// M:SpaceCenter.AutoPilot.Overshoot.
        /// real name: AutoPilot_set_AutoTune
        fn set_AutoTune(&self, value: Bool);

        /// The target time to peak used to autotune the PID controllers.
        /// A vector of three times, in seconds, for each of the pitch, roll
        /// and yaw axes.
        /// Defaults to 3 seconds for each axis.
        /// real name: AutoPilot_get_TimeToPeak
        fn get_TimeToPeak(&self) -> (Double, Double, Double,);

        /// The target time to peak used to autotune the PID controllers.
        /// A vector of three times, in seconds, for each of the pitch, roll
        /// and yaw axes.
        /// Defaults to 3 seconds for each axis.
        /// real name: AutoPilot_set_TimeToPeak
        fn set_TimeToPeak(&self, value: (Double, Double, Double,));

        /// The target overshoot percentage used to autotune the PID
        /// controllers.
        /// A vector of three values, between 0 and 1, for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 0.01 for each axis.
        /// real name: AutoPilot_get_Overshoot
        fn get_Overshoot(&self) -> (Double, Double, Double,);

        /// The target overshoot percentage used to autotune the PID
        /// controllers.
        /// A vector of three values, between 0 and 1, for each of the pitch,
        /// roll and yaw axes.
        /// Defaults to 0.01 for each axis.
        /// real name: AutoPilot_set_Overshoot
        fn set_Overshoot(&self, value: (Double, Double, Double,));

        /// Gains for the pitch PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_get_PitchPIDGains
        fn get_PitchPIDGains(&self) -> (Double, Double, Double,);

        /// Gains for the pitch PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_set_PitchPIDGains
        fn set_PitchPIDGains(&self, value: (Double, Double, Double,));

        /// Gains for the roll PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_get_RollPIDGains
        fn get_RollPIDGains(&self) -> (Double, Double, Double,);

        /// Gains for the roll PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_set_RollPIDGains
        fn set_RollPIDGains(&self, value: (Double, Double, Double,));

        /// Gains for the yaw PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_get_YawPIDGains
        fn get_YawPIDGains(&self) -> (Double, Double, Double,);

        /// Gains for the yaw PID controller.
        /// 
        /// # Remarks
        /// 
        /// When M:SpaceCenter.AutoPilot.AutoTune is true, these values are
        /// updated automatically,
        /// which will overwrite any manual changes.
        /// real name: AutoPilot_set_YawPIDGains
        fn set_YawPIDGains(&self, value: (Double, Double, Double,));

    }

    impl Camera {
        /// The current mode of the camera.
        /// real name: Camera_get_Mode
        fn get_Mode(&self) -> Enumeration;

        /// The current mode of the camera.
        /// real name: Camera_set_Mode
        fn set_Mode(&self, value: Enumeration);

        /// The pitch of the camera, in degrees.
        /// A value between M:SpaceCenter.Camera.MinPitch and
        /// M:SpaceCenter.Camera.MaxPitch
        /// real name: Camera_get_Pitch
        fn get_Pitch(&self) -> Float;

        /// The pitch of the camera, in degrees.
        /// A value between M:SpaceCenter.Camera.MinPitch and
        /// M:SpaceCenter.Camera.MaxPitch
        /// real name: Camera_set_Pitch
        fn set_Pitch(&self, value: Float);

        /// The heading of the camera, in degrees.
        /// real name: Camera_get_Heading
        fn get_Heading(&self) -> Float;

        /// The heading of the camera, in degrees.
        /// real name: Camera_set_Heading
        fn set_Heading(&self, value: Float);

        /// The distance from the camera to the subject, in meters.
        /// A value between M:SpaceCenter.Camera.MinDistance and
        /// M:SpaceCenter.Camera.MaxDistance.
        /// real name: Camera_get_Distance
        fn get_Distance(&self) -> Float;

        /// The distance from the camera to the subject, in meters.
        /// A value between M:SpaceCenter.Camera.MinDistance and
        /// M:SpaceCenter.Camera.MaxDistance.
        /// real name: Camera_set_Distance
        fn set_Distance(&self, value: Float);

        /// The minimum pitch of the camera.
        /// real name: Camera_get_MinPitch
        fn get_MinPitch(&self) -> Float;

        /// The maximum pitch of the camera.
        /// real name: Camera_get_MaxPitch
        fn get_MaxPitch(&self) -> Float;

        /// Minimum distance from the camera to the subject, in meters.
        /// real name: Camera_get_MinDistance
        fn get_MinDistance(&self) -> Float;

        /// Maximum distance from the camera to the subject, in meters.
        /// real name: Camera_get_MaxDistance
        fn get_MaxDistance(&self) -> Float;

        /// Default distance from the camera to the subject, in meters.
        /// real name: Camera_get_DefaultDistance
        fn get_DefaultDistance(&self) -> Float;

        /// In map mode, the celestial body that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a celestial body.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_get_FocussedBody
        fn get_FocussedBody(&self) -> Option<Class>;

        /// In map mode, the celestial body that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a celestial body.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_set_FocussedBody
        fn set_FocussedBody(&self, value: Class);

        /// In map mode, the vessel that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a vessel.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_get_FocussedVessel
        fn get_FocussedVessel(&self) -> Option<Class>;

        /// In map mode, the vessel that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a vessel.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_set_FocussedVessel
        fn set_FocussedVessel(&self, value: Class);

        /// In map mode, the maneuver node that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a maneuver node.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_get_FocussedNode
        fn get_FocussedNode(&self) -> Option<Class>;

        /// In map mode, the maneuver node that the camera is focussed on.
        /// Returns `null` if the camera is not focussed on a maneuver node.
        /// Returns an error is the camera is not in map mode.
        /// real name: Camera_set_FocussedNode
        fn set_FocussedNode(&self, value: Class);

    }

    impl CargoBay {
        /// The part object for this cargo bay.
        /// real name: CargoBay_get_Part
        fn get_Part(&self) -> Class;

        /// The state of the cargo bay.
        /// real name: CargoBay_get_State
        fn get_State(&self) -> Enumeration;

        /// Whether the cargo bay is open.
        /// real name: CargoBay_get_Open
        fn get_Open(&self) -> Bool;

        /// Whether the cargo bay is open.
        /// real name: CargoBay_set_Open
        fn set_Open(&self, value: Bool);

    }

    impl CelestialBody {
        /// The height of the surface relative to mean sea level, in meters,
        /// at the given position. When over water this is equal to 0.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// real name: CelestialBody_SurfaceHeight
        fn SurfaceHeight(&self, latitude: Double, longitude: Double) -> Double;

        /// The height of the surface relative to mean sea level, in meters,
        /// at the given position. When over water, this is the height
        /// of the sea-bed and is therefore  negative value.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// real name: CelestialBody_BedrockHeight
        fn BedrockHeight(&self, latitude: Double, longitude: Double) -> Double;

        /// The position at mean sea level at the given latitude and longitude,
        /// in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// Position as a vector.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// <param name="referenceFrame">Reference frame for the returned
        /// position vector.</param>
        /// real name: CelestialBody_MSLPosition
        fn MSLPosition(&self, latitude: Double, longitude: Double, referenceFrame: Class) -> (Double, Double, Double,);

        /// The position of the surface at the given latitude and longitude, in
        /// the given
        /// reference frame. When over water, this is the position of the
        /// surface of the water.
        /// 
        /// # Returns
        /// 
        /// Position as a vector.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// <param name="referenceFrame">Reference frame for the returned
        /// position vector.</param>
        /// real name: CelestialBody_SurfacePosition
        fn SurfacePosition(&self, latitude: Double, longitude: Double, referenceFrame: Class) -> (Double, Double, Double,);

        /// The position of the surface at the given latitude and longitude, in
        /// the given
        /// reference frame. When over water, this is the position at the
        /// bottom of the sea-bed.
        /// 
        /// # Returns
        /// 
        /// Position as a vector.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// <param name="referenceFrame">Reference frame for the returned
        /// position vector.</param>
        /// real name: CelestialBody_BedrockPosition
        fn BedrockPosition(&self, latitude: Double, longitude: Double, referenceFrame: Class) -> (Double, Double, Double,);

        /// The position at the given latitude, longitude and altitude, in the
        /// given reference frame.
        /// 
        /// # Returns
        /// 
        /// Position as a vector.
        /// 
        /// <param name="latitude">Latitude in degrees.</param>
        /// <param name="longitude">Longitude in degrees.</param>
        /// <param name="altitude">Altitude in meters above sea level.</param>
        /// <param name="referenceFrame">Reference frame for the returned
        /// position vector.</param>
        /// real name: CelestialBody_PositionAtAltitude
        fn PositionAtAltitude(&self, latitude: Double, longitude: Double, altitude: Double, referenceFrame: Class) -> (Double, Double, Double,);

        /// The latitude of the given position, in the given reference frame.
        /// 
        /// <param name="position">Position as a vector.</param>
        /// <param name="referenceFrame">Reference frame for the position
        /// vector.</param>
        /// real name: CelestialBody_LatitudeAtPosition
        fn LatitudeAtPosition(&self, position: (Double, Double, Double,), referenceFrame: Class) -> Double;

        /// The longitude of the given position, in the given reference frame.
        /// 
        /// <param name="position">Position as a vector.</param>
        /// <param name="referenceFrame">Reference frame for the position
        /// vector.</param>
        /// real name: CelestialBody_LongitudeAtPosition
        fn LongitudeAtPosition(&self, position: (Double, Double, Double,), referenceFrame: Class) -> Double;

        /// The altitude, in meters, of the given position in the given
        /// reference frame.
        /// 
        /// <param name="position">Position as a vector.</param>
        /// <param name="referenceFrame">Reference frame for the position
        /// vector.</param>
        /// real name: CelestialBody_AltitudeAtPosition
        fn AltitudeAtPosition(&self, position: (Double, Double, Double,), referenceFrame: Class) -> Double;

        /// The atmospheric density at the given position, in
        /// <math>kg/m^3</math>,
        /// in the given reference frame.
        /// 
        /// <param name="position">The position vector at which to measure the
        /// density.</param>
        /// <param name="referenceFrame">Reference frame that the position
        /// vector is in.</param>
        /// real name: CelestialBody_AtmosphericDensityAtPosition
        fn AtmosphericDensityAtPosition(&self, position: (Double, Double, Double,), referenceFrame: Class) -> Double;

        /// The temperature on the body at the given position, in the given
        /// reference frame.
        /// 
        /// <param name="position">Position as a vector.</param>
        /// <param name="referenceFrame">The reference frame that the position
        /// is in.</param>
        /// # Remarks
        /// 
        /// This calculation is performed using the bodies current position,
        /// which means that
        /// the value could be wrong if you want to know the temperature in the
        /// far future.
        /// real name: CelestialBody_TemperatureAt
        fn TemperatureAt(&self, position: (Double, Double, Double,), referenceFrame: Class) -> Double;

        /// Gets the air density, in <math>kg/m^3</math>, for the specified
        /// altitude above sea level, in meters.
        /// 
        /// # Remarks
        /// 
        /// This is an approximation, because actual calculations, taking sun
        /// exposure into account
        /// to compute air temperature, require us to know the exact point on
        /// the body where the
        /// density is to be computed (knowing the altitude is not enough).
        /// However, the difference is small for high altitudes, so it makes
        /// very little difference
        /// for trajectory prediction.
        /// real name: CelestialBody_DensityAt
        fn DensityAt(&self, altitude: Double) -> Double;

        /// Gets the air pressure, in Pascals, for the specified
        /// altitude above sea level, in meters.
        /// real name: CelestialBody_PressureAt
        fn PressureAt(&self, altitude: Double) -> Double;

        /// The biome at the given latitude and longitude, in degrees.
        /// real name: CelestialBody_BiomeAt
        fn BiomeAt(&self, latitude: Double, longitude: Double) -> String;

        /// The position of the center of the body, in the specified reference
        /// frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: CelestialBody_Position
        fn Position(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The linear velocity of the body, in the specified reference frame.
        /// 
        /// # Returns
        /// 
        /// The velocity as a vector. The vector points in the direction of
        /// travel,
        /// and its magnitude is the speed of the body in meters per second.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// velocity vector is in.</param>
        /// real name: CelestialBody_Velocity
        fn Velocity(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The rotation of the body, in the specified reference frame.
        /// 
        /// # Returns
        /// 
        /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// rotation is in.</param>
        /// real name: CelestialBody_Rotation
        fn Rotation(&self, referenceFrame: Class) -> (Double, Double, Double, Double,);

        /// The direction in which the north pole of the celestial body is
        /// pointing,
        /// in the specified reference frame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: CelestialBody_Direction
        fn Direction(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The angular velocity of the body in the specified reference frame.
        /// 
        /// # Returns
        /// 
        /// The angular velocity as a vector. The magnitude of the vector is
        /// the rotational
        /// speed of the body, in radians per second. The direction of the
        /// vector indicates the axis
        /// of rotation, using the right-hand rule.
        /// 
        /// <param name="referenceFrame">The reference frame the returned
        /// angular velocity is in.</param>
        /// real name: CelestialBody_AngularVelocity
        fn AngularVelocity(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The name of the body.
        /// real name: CelestialBody_get_Name
        fn get_Name(&self) -> String;

        /// A list of celestial bodies that are in orbit around this celestial
        /// body.
        /// real name: CelestialBody_get_Satellites
        fn get_Satellites(&self) -> List<Class>;

        /// The mass of the body, in kilograms.
        /// real name: CelestialBody_get_Mass
        fn get_Mass(&self) -> Float;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Standard_gravitational_parameter">standard
        /// gravitational parameter</a> of the body in <math>m^3s^{-2}</math>.
        /// real name: CelestialBody_get_GravitationalParameter
        fn get_GravitationalParameter(&self) -> Float;

        /// The acceleration due to gravity at sea level (mean altitude) on the
        /// body,
        /// in <math>m/s^2</math>.
        /// real name: CelestialBody_get_SurfaceGravity
        fn get_SurfaceGravity(&self) -> Float;

        /// The sidereal rotational period of the body, in seconds.
        /// real name: CelestialBody_get_RotationalPeriod
        fn get_RotationalPeriod(&self) -> Float;

        /// The rotational speed of the body, in radians per second.
        /// real name: CelestialBody_get_RotationalSpeed
        fn get_RotationalSpeed(&self) -> Float;

        /// The current rotation angle of the body, in radians.
        /// A value between 0 and <math>2\pi</math>
        /// real name: CelestialBody_get_RotationAngle
        fn get_RotationAngle(&self) -> Double;

        /// The initial rotation angle of the body (at UT 0), in radians.
        /// A value between 0 and <math>2\pi</math>
        /// real name: CelestialBody_get_InitialRotation
        fn get_InitialRotation(&self) -> Double;

        /// The equatorial radius of the body, in meters.
        /// real name: CelestialBody_get_EquatorialRadius
        fn get_EquatorialRadius(&self) -> Float;

        /// The radius of the sphere of influence of the body, in meters.
        /// real name: CelestialBody_get_SphereOfInfluence
        fn get_SphereOfInfluence(&self) -> Float;

        /// The orbit of the body.
        /// real name: CelestialBody_get_Orbit
        fn get_Orbit(&self) -> Option<Class>;

        /// `true` if the body has an atmosphere.
        /// real name: CelestialBody_get_HasAtmosphere
        fn get_HasAtmosphere(&self) -> Bool;

        /// The depth of the atmosphere, in meters.
        /// real name: CelestialBody_get_AtmosphereDepth
        fn get_AtmosphereDepth(&self) -> Float;

        /// `true` if there is oxygen in the atmosphere, required for
        /// air-breathing engines.
        /// real name: CelestialBody_get_HasAtmosphericOxygen
        fn get_HasAtmosphericOxygen(&self) -> Bool;

        /// The biomes present on this body.
        /// real name: CelestialBody_get_Biomes
        fn get_Biomes(&self) -> Set<>;

        /// The altitude, in meters, above which a vessel is considered to be
        /// flying "high" when doing science.
        /// real name: CelestialBody_get_FlyingHighAltitudeThreshold
        fn get_FlyingHighAltitudeThreshold(&self) -> Float;

        /// The altitude, in meters, above which a vessel is considered to be
        /// in "high" space when doing science.
        /// real name: CelestialBody_get_SpaceHighAltitudeThreshold
        fn get_SpaceHighAltitudeThreshold(&self) -> Float;

        /// The reference frame that is fixed relative to the celestial body.
        /// - The origin is at the center of the body.
        /// 
        /// - The axes rotate with the body.
        /// - The x-axis points from the center of the body
        /// towards the intersection of the prime meridian and equator (the
        /// position at 0° longitude, 0° latitude).
        /// - The y-axis points from the center of the body
        /// towards the north pole.
        /// - The z-axis points from the center of the body
        /// towards the equator at 90°E longitude.
        /// real name: CelestialBody_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to this celestial body,
        /// and
        /// orientated in a fixed direction (it does not rotate with the body).
        /// - The origin is at the center of the body.
        /// - The axes do not rotate.
        /// - The x-axis points in an arbitrary direction through the
        /// equator.
        /// - The y-axis points from the center of the body towards
        /// the north pole.
        /// - The z-axis points in an arbitrary direction through the
        /// equator.
        /// real name: CelestialBody_get_NonRotatingReferenceFrame
        fn get_NonRotatingReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to this celestial body,
        /// but
        /// orientated with the body's orbital prograde/normal/radial
        /// directions.
        /// - The origin is at the center of the body.
        /// 
        /// - The axes rotate with the orbital prograde/normal/radial
        /// directions.
        /// - The x-axis points in the orbital anti-radial direction.
        /// 
        /// - The y-axis points in the orbital prograde direction.
        /// 
        /// - The z-axis points in the orbital normal direction.
        /// real name: CelestialBody_get_OrbitalReferenceFrame
        fn get_OrbitalReferenceFrame(&self) -> Class;

    }

    impl CommLink {
        /// The type of link.
        /// real name: CommLink_get_Type
        fn get_Type(&self) -> Enumeration;

        /// Signal strength of the link.
        /// real name: CommLink_get_SignalStrength
        fn get_SignalStrength(&self) -> Double;

        /// Start point of the link.
        /// real name: CommLink_get_Start
        fn get_Start(&self) -> Class;

        /// Start point of the link.
        /// real name: CommLink_get_End
        fn get_End(&self) -> Class;

    }

    impl CommNode {
        /// Name of the communication node.
        /// real name: CommNode_get_Name
        fn get_Name(&self) -> String;

        /// Whether the communication node is on Kerbin.
        /// real name: CommNode_get_IsHome
        fn get_IsHome(&self) -> Bool;

        /// Whether the communication node is a control point, for example a
        /// manned vessel.
        /// real name: CommNode_get_IsControlPoint
        fn get_IsControlPoint(&self) -> Bool;

        /// Whether the communication node is a vessel.
        /// real name: CommNode_get_IsVessel
        fn get_IsVessel(&self) -> Bool;

        /// The vessel for this communication node.
        /// real name: CommNode_get_Vessel
        fn get_Vessel(&self) -> Class;

    }

    impl Comms {
        /// Whether the vessel can communicate with KSC.
        /// real name: Comms_get_CanCommunicate
        fn get_CanCommunicate(&self) -> Bool;

        /// Whether the vessel can transmit science data to KSC.
        /// real name: Comms_get_CanTransmitScience
        fn get_CanTransmitScience(&self) -> Bool;

        /// Signal strength to KSC.
        /// real name: Comms_get_SignalStrength
        fn get_SignalStrength(&self) -> Double;

        /// Signal delay to KSC in seconds.
        /// real name: Comms_get_SignalDelay
        fn get_SignalDelay(&self) -> Double;

        /// The combined power of all active antennae on the vessel.
        /// real name: Comms_get_Power
        fn get_Power(&self) -> Double;

        /// The communication path used to control the vessel.
        /// real name: Comms_get_ControlPath
        fn get_ControlPath(&self) -> List<Class>;

    }

    impl Contract {
        /// Cancel an active contract.
        /// real name: Contract_Cancel
        fn Cancel(&self);

        /// Accept an offered contract.
        /// real name: Contract_Accept
        fn Accept(&self);

        /// Decline an offered contract.
        /// real name: Contract_Decline
        fn Decline(&self);

        /// Type of the contract.
        /// real name: Contract_get_Type
        fn get_Type(&self) -> String;

        /// Title of the contract.
        /// real name: Contract_get_Title
        fn get_Title(&self) -> String;

        /// Description of the contract.
        /// real name: Contract_get_Description
        fn get_Description(&self) -> String;

        /// Notes for the contract.
        /// real name: Contract_get_Notes
        fn get_Notes(&self) -> String;

        /// Synopsis for the contract.
        /// real name: Contract_get_Synopsis
        fn get_Synopsis(&self) -> String;

        /// Keywords for the contract.
        /// real name: Contract_get_Keywords
        fn get_Keywords(&self) -> List<String>;

        /// State of the contract.
        /// real name: Contract_get_State
        fn get_State(&self) -> Enumeration;

        /// Whether the contract is active.
        /// real name: Contract_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the contract has been failed.
        /// real name: Contract_get_Failed
        fn get_Failed(&self) -> Bool;

        /// Whether the contract has been seen.
        /// real name: Contract_get_Seen
        fn get_Seen(&self) -> Bool;

        /// Whether the contract has been read.
        /// real name: Contract_get_Read
        fn get_Read(&self) -> Bool;

        /// Whether the contract can be canceled.
        /// real name: Contract_get_CanBeCanceled
        fn get_CanBeCanceled(&self) -> Bool;

        /// Whether the contract can be declined.
        /// real name: Contract_get_CanBeDeclined
        fn get_CanBeDeclined(&self) -> Bool;

        /// Whether the contract can be failed.
        /// real name: Contract_get_CanBeFailed
        fn get_CanBeFailed(&self) -> Bool;

        /// Funds received when accepting the contract.
        /// real name: Contract_get_FundsAdvance
        fn get_FundsAdvance(&self) -> Double;

        /// Funds received on completion of the contract.
        /// real name: Contract_get_FundsCompletion
        fn get_FundsCompletion(&self) -> Double;

        /// Funds lost if the contract is failed.
        /// real name: Contract_get_FundsFailure
        fn get_FundsFailure(&self) -> Double;

        /// Reputation gained on completion of the contract.
        /// real name: Contract_get_ReputationCompletion
        fn get_ReputationCompletion(&self) -> Double;

        /// Reputation lost if the contract is failed.
        /// real name: Contract_get_ReputationFailure
        fn get_ReputationFailure(&self) -> Double;

        /// Science gained on completion of the contract.
        /// real name: Contract_get_ScienceCompletion
        fn get_ScienceCompletion(&self) -> Double;

        /// Parameters for the contract.
        /// real name: Contract_get_Parameters
        fn get_Parameters(&self) -> List<Class>;

    }

    impl ContractManager {
        /// A list of all contract types.
        /// real name: ContractManager_get_Types
        fn get_Types(&self) -> Set<>;

        /// A list of all contracts.
        /// real name: ContractManager_get_AllContracts
        fn get_AllContracts(&self) -> List<Class>;

        /// A list of all active contracts.
        /// real name: ContractManager_get_ActiveContracts
        fn get_ActiveContracts(&self) -> List<Class>;

        /// A list of all offered, but unaccepted, contracts.
        /// real name: ContractManager_get_OfferedContracts
        fn get_OfferedContracts(&self) -> List<Class>;

        /// A list of all completed contracts.
        /// real name: ContractManager_get_CompletedContracts
        fn get_CompletedContracts(&self) -> List<Class>;

        /// A list of all failed contracts.
        /// real name: ContractManager_get_FailedContracts
        fn get_FailedContracts(&self) -> List<Class>;

    }

    impl ContractParameter {
        /// Title of the parameter.
        /// real name: ContractParameter_get_Title
        fn get_Title(&self) -> String;

        /// Notes for the parameter.
        /// real name: ContractParameter_get_Notes
        fn get_Notes(&self) -> String;

        /// Child contract parameters.
        /// real name: ContractParameter_get_Children
        fn get_Children(&self) -> List<Class>;

        /// Whether the parameter has been completed.
        /// real name: ContractParameter_get_Completed
        fn get_Completed(&self) -> Bool;

        /// Whether the parameter has been failed.
        /// real name: ContractParameter_get_Failed
        fn get_Failed(&self) -> Bool;

        /// Whether the contract parameter is optional.
        /// real name: ContractParameter_get_Optional
        fn get_Optional(&self) -> Bool;

        /// Funds received on completion of the contract parameter.
        /// real name: ContractParameter_get_FundsCompletion
        fn get_FundsCompletion(&self) -> Double;

        /// Funds lost if the contract parameter is failed.
        /// real name: ContractParameter_get_FundsFailure
        fn get_FundsFailure(&self) -> Double;

        /// Reputation gained on completion of the contract parameter.
        /// real name: ContractParameter_get_ReputationCompletion
        fn get_ReputationCompletion(&self) -> Double;

        /// Reputation lost if the contract parameter is failed.
        /// real name: ContractParameter_get_ReputationFailure
        fn get_ReputationFailure(&self) -> Double;

        /// Science gained on completion of the contract parameter.
        /// real name: ContractParameter_get_ScienceCompletion
        fn get_ScienceCompletion(&self) -> Double;

    }

    impl Control {
        /// Activates the next stage. Equivalent to pressing the space bar
        /// in-game.
        /// 
        /// # Returns
        /// 
        /// A list of vessel objects that are jettisoned from the active vessel.
        /// 
        /// # Remarks
        /// 
        /// When called, the active vessel may change. It is therefore possible
        /// that,
        /// after calling this function, the object(s) returned by previous
        /// call(s) to
        /// M:SpaceCenter.ActiveVessel no longer refer to the active vessel.
        /// real name: Control_ActivateNextStage
        fn ActivateNextStage(&self) -> List<Class>;

        /// Returns `true` if the given action group is enabled.
        /// 
        /// <param name="group">
        /// A number between 0 and 9 inclusive,
        /// or between 0 and 250 inclusive when the <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
        /// </param>
        /// real name: Control_GetActionGroup
        fn GetActionGroup(&self, group: Uint32) -> Bool;

        /// Sets the state of the given action group.
        /// 
        /// <param name="group">
        /// A number between 0 and 9 inclusive,
        /// or between 0 and 250 inclusive when the <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
        /// </param>
        /// <param name="state"></param>
        /// real name: Control_SetActionGroup
        fn SetActionGroup(&self, group: Uint32, state: Bool);

        /// Toggles the state of the given action group.
        /// 
        /// <param name="group">
        /// A number between 0 and 9 inclusive,
        /// or between 0 and 250 inclusive when the <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/67235-122dec1016-action-groups-extended-250-action-groups-in-flight-editing-now-kosremotetech/">Extended Action Groups mod</a> is installed.
        /// </param>
        /// real name: Control_ToggleActionGroup
        fn ToggleActionGroup(&self, group: Uint32);

        /// Creates a maneuver node at the given universal time, and returns a
        /// T:SpaceCenter.Node object that can be used to modify it.
        /// Optionally sets the magnitude of the delta-v for the maneuver node
        /// in the prograde, normal and radial directions.
        /// 
        /// <param name="ut">Universal time of the maneuver node.</param>
        /// <param name="prograde">Delta-v in the prograde direction.</param>
        /// <param name="normal">Delta-v in the normal direction.</param>
        /// <param name="radial">Delta-v in the radial direction.</param>
        /// real name: Control_AddNode
        fn AddNode(&self, ut: Double, prograde: Float, normal: Float, radial: Float) -> Class;

        /// Remove all maneuver nodes.
        /// real name: Control_RemoveNodes
        fn RemoveNodes(&self);

        /// The control state of the vessel.
        /// real name: Control_get_State
        fn get_State(&self) -> Enumeration;

        /// The source of the vessels control, for example by a kerbal or a
        /// probe core.
        /// real name: Control_get_Source
        fn get_Source(&self) -> Enumeration;

        /// The state of SAS.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.AutoPilot.SAS
        /// real name: Control_get_SAS
        fn get_SAS(&self) -> Bool;

        /// The state of SAS.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.AutoPilot.SAS
        /// real name: Control_set_SAS
        fn set_SAS(&self, value: Bool);

        /// The current T:SpaceCenter.SASMode.
        /// These modes are equivalent to the mode buttons to
        /// the left of the navball that appear when SAS is enabled.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.AutoPilot.SASMode
        /// real name: Control_get_SASMode
        fn get_SASMode(&self) -> Enumeration;

        /// The current T:SpaceCenter.SASMode.
        /// These modes are equivalent to the mode buttons to
        /// the left of the navball that appear when SAS is enabled.
        /// 
        /// # Remarks
        /// Equivalent to M:SpaceCenter.AutoPilot.SASMode
        /// real name: Control_set_SASMode
        fn set_SASMode(&self, value: Enumeration);

        /// The current T:SpaceCenter.SpeedMode of the navball.
        /// This is the mode displayed next to the speed at the top of the
        /// navball.
        /// real name: Control_get_SpeedMode
        fn get_SpeedMode(&self) -> Enumeration;

        /// The current T:SpaceCenter.SpeedMode of the navball.
        /// This is the mode displayed next to the speed at the top of the
        /// navball.
        /// real name: Control_set_SpeedMode
        fn set_SpeedMode(&self, value: Enumeration);

        /// The state of RCS.
        /// real name: Control_get_RCS
        fn get_RCS(&self) -> Bool;

        /// The state of RCS.
        /// real name: Control_set_RCS
        fn set_RCS(&self, value: Bool);

        /// Returns whether all reactive wheels on the vessel are active,
        /// and sets the active state of all reaction wheels.
        /// See M:SpaceCenter.ReactionWheel.Active.
        /// real name: Control_get_ReactionWheels
        fn get_ReactionWheels(&self) -> Bool;

        /// Returns whether all reactive wheels on the vessel are active,
        /// and sets the active state of all reaction wheels.
        /// See M:SpaceCenter.ReactionWheel.Active.
        /// real name: Control_set_ReactionWheels
        fn set_ReactionWheels(&self, value: Bool);

        /// The state of the landing gear/legs.
        /// real name: Control_get_Gear
        fn get_Gear(&self) -> Bool;

        /// The state of the landing gear/legs.
        /// real name: Control_set_Gear
        fn set_Gear(&self, value: Bool);

        /// Returns whether all landing legs on the vessel are deployed,
        /// and sets the deployment state of all landing legs.
        /// Does not include wheels (for example landing gear).
        /// See M:SpaceCenter.Leg.Deployed.
        /// real name: Control_get_Legs
        fn get_Legs(&self) -> Bool;

        /// Returns whether all landing legs on the vessel are deployed,
        /// and sets the deployment state of all landing legs.
        /// Does not include wheels (for example landing gear).
        /// See M:SpaceCenter.Leg.Deployed.
        /// real name: Control_set_Legs
        fn set_Legs(&self, value: Bool);

        /// Returns whether all wheels on the vessel are deployed,
        /// and sets the deployment state of all wheels.
        /// Does not include landing legs.
        /// See M:SpaceCenter.Wheel.Deployed.
        /// real name: Control_get_Wheels
        fn get_Wheels(&self) -> Bool;

        /// Returns whether all wheels on the vessel are deployed,
        /// and sets the deployment state of all wheels.
        /// Does not include landing legs.
        /// See M:SpaceCenter.Wheel.Deployed.
        /// real name: Control_set_Wheels
        fn set_Wheels(&self, value: Bool);

        /// The state of the lights.
        /// real name: Control_get_Lights
        fn get_Lights(&self) -> Bool;

        /// The state of the lights.
        /// real name: Control_set_Lights
        fn set_Lights(&self, value: Bool);

        /// The state of the wheel brakes.
        /// real name: Control_get_Brakes
        fn get_Brakes(&self) -> Bool;

        /// The state of the wheel brakes.
        /// real name: Control_set_Brakes
        fn set_Brakes(&self, value: Bool);

        /// Returns whether all antennas on the vessel are deployed,
        /// and sets the deployment state of all antennas.
        /// See M:SpaceCenter.Antenna.Deployed.
        /// real name: Control_get_Antennas
        fn get_Antennas(&self) -> Bool;

        /// Returns whether all antennas on the vessel are deployed,
        /// and sets the deployment state of all antennas.
        /// See M:SpaceCenter.Antenna.Deployed.
        /// real name: Control_set_Antennas
        fn set_Antennas(&self, value: Bool);

        /// Returns whether any of the cargo bays on the vessel are open,
        /// and sets the open state of all cargo bays.
        /// See M:SpaceCenter.CargoBay.Open.
        /// real name: Control_get_CargoBays
        fn get_CargoBays(&self) -> Bool;

        /// Returns whether any of the cargo bays on the vessel are open,
        /// and sets the open state of all cargo bays.
        /// See M:SpaceCenter.CargoBay.Open.
        /// real name: Control_set_CargoBays
        fn set_CargoBays(&self, value: Bool);

        /// Returns whether all of the air intakes on the vessel are open,
        /// and sets the open state of all air intakes.
        /// See M:SpaceCenter.Intake.Open.
        /// real name: Control_get_Intakes
        fn get_Intakes(&self) -> Bool;

        /// Returns whether all of the air intakes on the vessel are open,
        /// and sets the open state of all air intakes.
        /// See M:SpaceCenter.Intake.Open.
        /// real name: Control_set_Intakes
        fn set_Intakes(&self, value: Bool);

        /// Returns whether all parachutes on the vessel are deployed,
        /// and sets the deployment state of all parachutes.
        /// Cannot be set to `false`.
        /// See M:SpaceCenter.Parachute.Deployed.
        /// real name: Control_get_Parachutes
        fn get_Parachutes(&self) -> Bool;

        /// Returns whether all parachutes on the vessel are deployed,
        /// and sets the deployment state of all parachutes.
        /// Cannot be set to `false`.
        /// See M:SpaceCenter.Parachute.Deployed.
        /// real name: Control_set_Parachutes
        fn set_Parachutes(&self, value: Bool);

        /// Returns whether all radiators on the vessel are deployed,
        /// and sets the deployment state of all radiators.
        /// See M:SpaceCenter.Radiator.Deployed.
        /// real name: Control_get_Radiators
        fn get_Radiators(&self) -> Bool;

        /// Returns whether all radiators on the vessel are deployed,
        /// and sets the deployment state of all radiators.
        /// See M:SpaceCenter.Radiator.Deployed.
        /// real name: Control_set_Radiators
        fn set_Radiators(&self, value: Bool);

        /// Returns whether all of the resource harvesters on the vessel are
        /// deployed,
        /// and sets the deployment state of all resource harvesters.
        /// See M:SpaceCenter.ResourceHarvester.Deployed.
        /// real name: Control_get_ResourceHarvesters
        fn get_ResourceHarvesters(&self) -> Bool;

        /// Returns whether all of the resource harvesters on the vessel are
        /// deployed,
        /// and sets the deployment state of all resource harvesters.
        /// See M:SpaceCenter.ResourceHarvester.Deployed.
        /// real name: Control_set_ResourceHarvesters
        fn set_ResourceHarvesters(&self, value: Bool);

        /// Returns whether any of the resource harvesters on the vessel are
        /// active,
        /// and sets the active state of all resource harvesters.
        /// See M:SpaceCenter.ResourceHarvester.Active.
        /// real name: Control_get_ResourceHarvestersActive
        fn get_ResourceHarvestersActive(&self) -> Bool;

        /// Returns whether any of the resource harvesters on the vessel are
        /// active,
        /// and sets the active state of all resource harvesters.
        /// See M:SpaceCenter.ResourceHarvester.Active.
        /// real name: Control_set_ResourceHarvestersActive
        fn set_ResourceHarvestersActive(&self, value: Bool);

        /// Returns whether all solar panels on the vessel are deployed,
        /// and sets the deployment state of all solar panels.
        /// See M:SpaceCenter.SolarPanel.Deployed.
        /// real name: Control_get_SolarPanels
        fn get_SolarPanels(&self) -> Bool;

        /// Returns whether all solar panels on the vessel are deployed,
        /// and sets the deployment state of all solar panels.
        /// See M:SpaceCenter.SolarPanel.Deployed.
        /// real name: Control_set_SolarPanels
        fn set_SolarPanels(&self, value: Bool);

        /// The state of the abort action group.
        /// real name: Control_get_Abort
        fn get_Abort(&self) -> Bool;

        /// The state of the abort action group.
        /// real name: Control_set_Abort
        fn set_Abort(&self, value: Bool);

        /// The state of the throttle. A value between 0 and 1.
        /// real name: Control_get_Throttle
        fn get_Throttle(&self) -> Float;

        /// The state of the throttle. A value between 0 and 1.
        /// real name: Control_set_Throttle
        fn set_Throttle(&self, value: Float);

        /// Sets the behavior of the pitch, yaw, roll and translation control
        /// inputs.
        /// When set to additive, these inputs are added to the vessels current
        /// inputs.
        /// This mode is the default.
        /// When set to override, these inputs (if non-zero) override the
        /// vessels inputs.
        /// This mode prevents keyboard control, or SAS, from interfering with
        /// the controls when
        /// they are set.
        /// real name: Control_get_InputMode
        fn get_InputMode(&self) -> Enumeration;

        /// Sets the behavior of the pitch, yaw, roll and translation control
        /// inputs.
        /// When set to additive, these inputs are added to the vessels current
        /// inputs.
        /// This mode is the default.
        /// When set to override, these inputs (if non-zero) override the
        /// vessels inputs.
        /// This mode prevents keyboard control, or SAS, from interfering with
        /// the controls when
        /// they are set.
        /// real name: Control_set_InputMode
        fn set_InputMode(&self, value: Enumeration);

        /// The state of the pitch control.
        /// A value between -1 and 1.
        /// Equivalent to the w and s keys.
        /// real name: Control_get_Pitch
        fn get_Pitch(&self) -> Float;

        /// The state of the pitch control.
        /// A value between -1 and 1.
        /// Equivalent to the w and s keys.
        /// real name: Control_set_Pitch
        fn set_Pitch(&self, value: Float);

        /// The state of the yaw control.
        /// A value between -1 and 1.
        /// Equivalent to the a and d keys.
        /// real name: Control_get_Yaw
        fn get_Yaw(&self) -> Float;

        /// The state of the yaw control.
        /// A value between -1 and 1.
        /// Equivalent to the a and d keys.
        /// real name: Control_set_Yaw
        fn set_Yaw(&self, value: Float);

        /// The state of the roll control.
        /// A value between -1 and 1.
        /// Equivalent to the q and e keys.
        /// real name: Control_get_Roll
        fn get_Roll(&self) -> Float;

        /// The state of the roll control.
        /// A value between -1 and 1.
        /// Equivalent to the q and e keys.
        /// real name: Control_set_Roll
        fn set_Roll(&self, value: Float);

        /// The state of the forward translational control.
        /// A value between -1 and 1.
        /// Equivalent to the h and n keys.
        /// real name: Control_get_Forward
        fn get_Forward(&self) -> Float;

        /// The state of the forward translational control.
        /// A value between -1 and 1.
        /// Equivalent to the h and n keys.
        /// real name: Control_set_Forward
        fn set_Forward(&self, value: Float);

        /// The state of the up translational control.
        /// A value between -1 and 1.
        /// Equivalent to the i and k keys.
        /// real name: Control_get_Up
        fn get_Up(&self) -> Float;

        /// The state of the up translational control.
        /// A value between -1 and 1.
        /// Equivalent to the i and k keys.
        /// real name: Control_set_Up
        fn set_Up(&self, value: Float);

        /// The state of the right translational control.
        /// A value between -1 and 1.
        /// Equivalent to the j and l keys.
        /// real name: Control_get_Right
        fn get_Right(&self) -> Float;

        /// The state of the right translational control.
        /// A value between -1 and 1.
        /// Equivalent to the j and l keys.
        /// real name: Control_set_Right
        fn set_Right(&self, value: Float);

        /// The state of the wheel throttle.
        /// A value between -1 and 1.
        /// A value of 1 rotates the wheels forwards, a value of -1 rotates
        /// the wheels backwards.
        /// real name: Control_get_WheelThrottle
        fn get_WheelThrottle(&self) -> Float;

        /// The state of the wheel throttle.
        /// A value between -1 and 1.
        /// A value of 1 rotates the wheels forwards, a value of -1 rotates
        /// the wheels backwards.
        /// real name: Control_set_WheelThrottle
        fn set_WheelThrottle(&self, value: Float);

        /// The state of the wheel steering.
        /// A value between -1 and 1.
        /// A value of 1 steers to the left, and a value of -1 steers to the
        /// right.
        /// real name: Control_get_WheelSteering
        fn get_WheelSteering(&self) -> Float;

        /// The state of the wheel steering.
        /// A value between -1 and 1.
        /// A value of 1 steers to the left, and a value of -1 steers to the
        /// right.
        /// real name: Control_set_WheelSteering
        fn set_WheelSteering(&self, value: Float);

        /// The current stage of the vessel. Corresponds to the stage number in
        /// the in-game UI.
        /// real name: Control_get_CurrentStage
        fn get_CurrentStage(&self) -> Sint32;

        /// Returns a list of all existing maneuver nodes, ordered by time from
        /// first to last.
        /// real name: Control_get_Nodes
        fn get_Nodes(&self) -> List<Class>;

    }

    impl ControlSurface {
        /// The part object for this control surface.
        /// real name: ControlSurface_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the control surface has pitch control enabled.
        /// real name: ControlSurface_get_PitchEnabled
        fn get_PitchEnabled(&self) -> Bool;

        /// Whether the control surface has pitch control enabled.
        /// real name: ControlSurface_set_PitchEnabled
        fn set_PitchEnabled(&self, value: Bool);

        /// Whether the control surface has yaw control enabled.
        /// real name: ControlSurface_get_YawEnabled
        fn get_YawEnabled(&self) -> Bool;

        /// Whether the control surface has yaw control enabled.
        /// real name: ControlSurface_set_YawEnabled
        fn set_YawEnabled(&self, value: Bool);

        /// Whether the control surface has roll control enabled.
        /// real name: ControlSurface_get_RollEnabled
        fn get_RollEnabled(&self) -> Bool;

        /// Whether the control surface has roll control enabled.
        /// real name: ControlSurface_set_RollEnabled
        fn set_RollEnabled(&self, value: Bool);

        /// The authority limiter for the control surface, which controls how
        /// far the
        /// control surface will move.
        /// real name: ControlSurface_get_AuthorityLimiter
        fn get_AuthorityLimiter(&self) -> Float;

        /// The authority limiter for the control surface, which controls how
        /// far the
        /// control surface will move.
        /// real name: ControlSurface_set_AuthorityLimiter
        fn set_AuthorityLimiter(&self, value: Float);

        /// Whether the control surface movement is inverted.
        /// real name: ControlSurface_get_Inverted
        fn get_Inverted(&self) -> Bool;

        /// Whether the control surface movement is inverted.
        /// real name: ControlSurface_set_Inverted
        fn set_Inverted(&self, value: Bool);

        /// Whether the control surface has been fully deployed.
        /// real name: ControlSurface_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the control surface has been fully deployed.
        /// real name: ControlSurface_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// Surface area of the control surface in <math>m^2</math>.
        /// real name: ControlSurface_get_SurfaceArea
        fn get_SurfaceArea(&self) -> Float;

        /// The available torque, in Newton meters, that can be produced by
        /// this control surface,
        /// in the positive and negative pitch, roll and yaw axes of the
        /// vessel. These axes
        /// correspond to the coordinate axes of the
        /// M:SpaceCenter.Vessel.ReferenceFrame.
        /// real name: ControlSurface_get_AvailableTorque
        fn get_AvailableTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

    }

    impl CrewMember {
        /// The crew members name.
        /// real name: CrewMember_get_Name
        fn get_Name(&self) -> String;

        /// The crew members name.
        /// real name: CrewMember_set_Name
        fn set_Name(&self, value: String);

        /// The type of crew member.
        /// real name: CrewMember_get_Type
        fn get_Type(&self) -> Enumeration;

        /// Whether the crew member is on a mission.
        /// real name: CrewMember_get_OnMission
        fn get_OnMission(&self) -> Bool;

        /// The crew members courage.
        /// real name: CrewMember_get_Courage
        fn get_Courage(&self) -> Float;

        /// The crew members courage.
        /// real name: CrewMember_set_Courage
        fn set_Courage(&self, value: Float);

        /// The crew members stupidity.
        /// real name: CrewMember_get_Stupidity
        fn get_Stupidity(&self) -> Float;

        /// The crew members stupidity.
        /// real name: CrewMember_set_Stupidity
        fn set_Stupidity(&self, value: Float);

        /// The crew members experience.
        /// real name: CrewMember_get_Experience
        fn get_Experience(&self) -> Float;

        /// The crew members experience.
        /// real name: CrewMember_set_Experience
        fn set_Experience(&self, value: Float);

        /// Whether the crew member is a badass.
        /// real name: CrewMember_get_Badass
        fn get_Badass(&self) -> Bool;

        /// Whether the crew member is a badass.
        /// real name: CrewMember_set_Badass
        fn set_Badass(&self, value: Bool);

        /// Whether the crew member is a veteran.
        /// real name: CrewMember_get_Veteran
        fn get_Veteran(&self) -> Bool;

        /// Whether the crew member is a veteran.
        /// real name: CrewMember_set_Veteran
        fn set_Veteran(&self, value: Bool);

    }

    impl Decoupler {
        /// Fires the decoupler. Returns the new vessel created when the
        /// decoupler fires.
        /// Throws an exception if the decoupler has already fired.
        /// 
        /// # Remarks
        /// 
        /// When called, the active vessel may change. It is therefore possible
        /// that,
        /// after calling this function, the object(s) returned by previous
        /// call(s) to
        /// M:SpaceCenter.ActiveVessel no longer refer to the active vessel.
        /// real name: Decoupler_Decouple
        fn Decouple(&self) -> Class;

        /// The part object for this decoupler.
        /// real name: Decoupler_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the decoupler has fired.
        /// real name: Decoupler_get_Decoupled
        fn get_Decoupled(&self) -> Bool;

        /// Whether the decoupler is enabled in the staging sequence.
        /// real name: Decoupler_get_Staged
        fn get_Staged(&self) -> Bool;

        /// The impulse that the decoupler imparts when it is fired, in Newton
        /// seconds.
        /// real name: Decoupler_get_Impulse
        fn get_Impulse(&self) -> Float;

    }

    impl DockingPort {
        /// Undocks the docking port and returns the new T:SpaceCenter.Vessel
        /// that is created.
        /// This method can be called for either docking port in a docked pair.
        /// Throws an exception if the docking port is not docked to anything.
        /// 
        /// # Remarks
        /// 
        /// When called, the active vessel may change. It is therefore possible
        /// that,
        /// after calling this function, the object(s) returned by previous
        /// call(s) to
        /// M:SpaceCenter.ActiveVessel no longer refer to the active vessel.
        /// real name: DockingPort_Undock
        fn Undock(&self) -> Class;

        /// The position of the docking port, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: DockingPort_Position
        fn Position(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The direction that docking port points in, in the given reference
        /// frame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: DockingPort_Direction
        fn Direction(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The rotation of the docking port, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// rotation is in.</param>
        /// real name: DockingPort_Rotation
        fn Rotation(&self, referenceFrame: Class) -> (Double, Double, Double, Double,);

        /// The part object for this docking port.
        /// real name: DockingPort_get_Part
        fn get_Part(&self) -> Class;

        /// The current state of the docking port.
        /// real name: DockingPort_get_State
        fn get_State(&self) -> Enumeration;

        /// The part that this docking port is docked to. Returns `null` if this
        /// docking port is not docked to anything.
        /// real name: DockingPort_get_DockedPart
        fn get_DockedPart(&self) -> Option<Class>;

        /// The distance a docking port must move away when it undocks before it
        /// becomes ready to dock with another port, in meters.
        /// real name: DockingPort_get_ReengageDistance
        fn get_ReengageDistance(&self) -> Float;

        /// Whether the docking port has a shield.
        /// real name: DockingPort_get_HasShield
        fn get_HasShield(&self) -> Bool;

        /// The state of the docking ports shield, if it has one.
        /// 
        /// Returns `true` if the docking port has a shield, and the shield is
        /// closed. Otherwise returns `false`. When set to `true`, the shield is
        /// closed, and when set to `false` the shield is opened. If the docking
        /// port does not have a shield, setting this attribute has no effect.
        /// real name: DockingPort_get_Shielded
        fn get_Shielded(&self) -> Bool;

        /// The state of the docking ports shield, if it has one.
        /// 
        /// Returns `true` if the docking port has a shield, and the shield is
        /// closed. Otherwise returns `false`. When set to `true`, the shield is
        /// closed, and when set to `false` the shield is opened. If the docking
        /// port does not have a shield, setting this attribute has no effect.
        /// real name: DockingPort_set_Shielded
        fn set_Shielded(&self, value: Bool);

        /// The reference frame that is fixed relative to this docking port, and
        /// oriented with the port.
        /// - The origin is at the position of the docking port.
        /// 
        /// - The axes rotate with the docking port.
        /// - The x-axis points out to the right side of the docking port.
        /// 
        /// - The y-axis points in the direction the docking port is facing.
        /// 
        /// - The z-axis points out of the bottom off the docking port.
        /// 
        /// 
        /// # Remarks
        /// 
        /// This reference frame is not necessarily equivalent to the reference
        /// frame
        /// for the part, returned by M:SpaceCenter.Part.ReferenceFrame.
        /// real name: DockingPort_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

    }

    impl Engine {
        /// Toggle the current engine mode.
        /// real name: Engine_ToggleMode
        fn ToggleMode(&self);

        /// The part object for this engine.
        /// real name: Engine_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the engine is active. Setting this attribute may have no
        /// effect,
        /// depending on M:SpaceCenter.Engine.CanShutdown and
        /// M:SpaceCenter.Engine.CanRestart.
        /// real name: Engine_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the engine is active. Setting this attribute may have no
        /// effect,
        /// depending on M:SpaceCenter.Engine.CanShutdown and
        /// M:SpaceCenter.Engine.CanRestart.
        /// real name: Engine_set_Active
        fn set_Active(&self, value: Bool);

        /// The current amount of thrust being produced by the engine, in
        /// Newtons.
        /// real name: Engine_get_Thrust
        fn get_Thrust(&self) -> Float;

        /// The amount of thrust, in Newtons, that would be produced by the
        /// engine
        /// when activated and with its throttle set to 100%.
        /// Returns zero if the engine does not have any fuel.
        /// Takes the engine's current M:SpaceCenter.Engine.ThrustLimit and
        /// atmospheric conditions
        /// into account.
        /// real name: Engine_get_AvailableThrust
        fn get_AvailableThrust(&self) -> Float;

        /// The amount of thrust, in Newtons, that would be produced by the
        /// engine
        /// when activated and fueled, with its throttle and throttle limiter
        /// set to 100%.
        /// real name: Engine_get_MaxThrust
        fn get_MaxThrust(&self) -> Float;

        /// The maximum amount of thrust that can be produced by the engine in a
        /// vacuum, in Newtons. This is the amount of thrust produced by the
        /// engine
        /// when activated, M:SpaceCenter.Engine.ThrustLimit is set to 100%,
        /// the main
        /// vessel's throttle is set to 100% and the engine is in a vacuum.
        /// real name: Engine_get_MaxVacuumThrust
        fn get_MaxVacuumThrust(&self) -> Float;

        /// The thrust limiter of the engine. A value between 0 and 1. Setting
        /// this
        /// attribute may have no effect, for example the thrust limit for a
        /// solid
        /// rocket booster cannot be changed in flight.
        /// real name: Engine_get_ThrustLimit
        fn get_ThrustLimit(&self) -> Float;

        /// The thrust limiter of the engine. A value between 0 and 1. Setting
        /// this
        /// attribute may have no effect, for example the thrust limit for a
        /// solid
        /// rocket booster cannot be changed in flight.
        /// real name: Engine_set_ThrustLimit
        fn set_ThrustLimit(&self, value: Float);

        /// The components of the engine that generate thrust.
        /// 
        /// # Remarks
        /// 
        /// For example, this corresponds to the rocket nozzel on a solid
        /// rocket booster,
        /// or the individual nozzels on a RAPIER engine.
        /// The overall thrust produced by the engine, as reported by
        /// M:SpaceCenter.Engine.AvailableThrust,
        /// M:SpaceCenter.Engine.MaxThrust and others, is the sum of the thrust
        /// generated by each thruster.
        /// real name: Engine_get_Thrusters
        fn get_Thrusters(&self) -> List<Class>;

        /// The current specific impulse of the engine, in seconds. Returns zero
        /// if the engine is not active.
        /// real name: Engine_get_SpecificImpulse
        fn get_SpecificImpulse(&self) -> Float;

        /// The vacuum specific impulse of the engine, in seconds.
        /// real name: Engine_get_VacuumSpecificImpulse
        fn get_VacuumSpecificImpulse(&self) -> Float;

        /// The specific impulse of the engine at sea level on Kerbin, in
        /// seconds.
        /// real name: Engine_get_KerbinSeaLevelSpecificImpulse
        fn get_KerbinSeaLevelSpecificImpulse(&self) -> Float;

        /// The names of the propellants that the engine consumes.
        /// real name: Engine_get_PropellantNames
        fn get_PropellantNames(&self) -> List<String>;

        /// The propellants that the engine consumes.
        /// real name: Engine_get_Propellants
        fn get_Propellants(&self) -> List<Class>;

        /// The ratio of resources that the engine consumes. A dictionary
        /// mapping resource names
        /// to the ratio at which they are consumed by the engine.
        /// 
        /// # Remarks
        /// 
        /// For example, if the ratios are 0.6 for LiquidFuel and 0.4 for
        /// Oxidizer, then for every
        /// 0.6 units of LiquidFuel that the engine burns, it will burn 0.4
        /// units of Oxidizer.
        /// real name: Engine_get_PropellantRatios
        fn get_PropellantRatios(&self) -> Dictionary<>;

        /// Whether the engine has any fuel available.
        /// 
        /// # Remarks
        /// 
        /// The engine must be activated for this property to update correctly.
        /// real name: Engine_get_HasFuel
        fn get_HasFuel(&self) -> Bool;

        /// The current throttle setting for the engine. A value between 0 and
        /// 1.
        /// This is not necessarily the same as the vessel's main throttle
        /// setting, as some engines take time to adjust their throttle
        /// (such as jet engines).
        /// real name: Engine_get_Throttle
        fn get_Throttle(&self) -> Float;

        /// Whether the M:SpaceCenter.Control.Throttle affects the engine. For
        /// example,
        /// this is `true` for liquid fueled rockets, and `false` for solid
        /// rocket
        /// boosters.
        /// real name: Engine_get_ThrottleLocked
        fn get_ThrottleLocked(&self) -> Bool;

        /// Whether the engine can be restarted once shutdown. If the engine
        /// cannot be shutdown,
        /// returns `false`. For example, this is `true` for liquid fueled
        /// rockets
        /// and `false` for solid rocket boosters.
        /// real name: Engine_get_CanRestart
        fn get_CanRestart(&self) -> Bool;

        /// Whether the engine can be shutdown once activated. For example,
        /// this is
        /// `true` for liquid fueled rockets and `false` for solid rocket
        /// boosters.
        /// real name: Engine_get_CanShutdown
        fn get_CanShutdown(&self) -> Bool;

        /// Whether the engine has multiple modes of operation.
        /// real name: Engine_get_HasModes
        fn get_HasModes(&self) -> Bool;

        /// The name of the current engine mode.
        /// real name: Engine_get_Mode
        fn get_Mode(&self) -> String;

        /// The name of the current engine mode.
        /// real name: Engine_set_Mode
        fn set_Mode(&self, value: String);

        /// The available modes for the engine.
        /// A dictionary mapping mode names to T:SpaceCenter.Engine objects.
        /// real name: Engine_get_Modes
        fn get_Modes(&self) -> Dictionary<>;

        /// Whether the engine will automatically switch modes.
        /// real name: Engine_get_AutoModeSwitch
        fn get_AutoModeSwitch(&self) -> Bool;

        /// Whether the engine will automatically switch modes.
        /// real name: Engine_set_AutoModeSwitch
        fn set_AutoModeSwitch(&self, value: Bool);

        /// Whether the engine is gimballed.
        /// real name: Engine_get_Gimballed
        fn get_Gimballed(&self) -> Bool;

        /// The range over which the gimbal can move, in degrees.
        /// Returns 0 if the engine is not gimballed.
        /// real name: Engine_get_GimbalRange
        fn get_GimbalRange(&self) -> Float;

        /// Whether the engines gimbal is locked in place. Setting this
        /// attribute has
        /// no effect if the engine is not gimballed.
        /// real name: Engine_get_GimbalLocked
        fn get_GimbalLocked(&self) -> Bool;

        /// Whether the engines gimbal is locked in place. Setting this
        /// attribute has
        /// no effect if the engine is not gimballed.
        /// real name: Engine_set_GimbalLocked
        fn set_GimbalLocked(&self, value: Bool);

        /// The gimbal limiter of the engine. A value between 0 and 1.
        /// Returns 0 if the gimbal is locked.
        /// real name: Engine_get_GimbalLimit
        fn get_GimbalLimit(&self) -> Float;

        /// The gimbal limiter of the engine. A value between 0 and 1.
        /// Returns 0 if the gimbal is locked.
        /// real name: Engine_set_GimbalLimit
        fn set_GimbalLimit(&self, value: Float);

        /// The available torque, in Newton meters, that can be produced by
        /// this engine,
        /// in the positive and negative pitch, roll and yaw axes of the
        /// vessel. These axes
        /// correspond to the coordinate axes of the
        /// M:SpaceCenter.Vessel.ReferenceFrame.
        /// Returns zero if the engine is inactive, or not gimballed.
        /// real name: Engine_get_AvailableTorque
        fn get_AvailableTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

    }

    impl Experiment {
        /// Run the experiment.
        /// real name: Experiment_Run
        fn Run(&self);

        /// Transmit all experimental data contained by this part.
        /// real name: Experiment_Transmit
        fn Transmit(&self);

        /// Dump the experimental data contained by the experiment.
        /// real name: Experiment_Dump
        fn Dump(&self);

        /// Reset the experiment.
        /// real name: Experiment_Reset
        fn Reset(&self);

        /// The part object for this experiment.
        /// real name: Experiment_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the experiment is inoperable.
        /// real name: Experiment_get_Inoperable
        fn get_Inoperable(&self) -> Bool;

        /// Whether the experiment has been deployed.
        /// real name: Experiment_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the experiment can be re-run.
        /// real name: Experiment_get_Rerunnable
        fn get_Rerunnable(&self) -> Bool;

        /// Whether the experiment contains data.
        /// real name: Experiment_get_HasData
        fn get_HasData(&self) -> Bool;

        /// The data contained in this experiment.
        /// real name: Experiment_get_Data
        fn get_Data(&self) -> List<Class>;

        /// Determines if the experiment is available given the current
        /// conditions.
        /// real name: Experiment_get_Available
        fn get_Available(&self) -> Bool;

        /// The name of the biome the experiment is currently in.
        /// real name: Experiment_get_Biome
        fn get_Biome(&self) -> String;

        /// Containing information on the corresponding specific science result
        /// for the current
        /// conditions. Returns `null` if the experiment is unavailable.
        /// real name: Experiment_get_ScienceSubject
        fn get_ScienceSubject(&self) -> Class;

    }

    impl Fairing {
        /// Jettison the fairing. Has no effect if it has already been
        /// jettisoned.
        /// real name: Fairing_Jettison
        fn Jettison(&self);

        /// The part object for this fairing.
        /// real name: Fairing_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the fairing has been jettisoned.
        /// real name: Fairing_get_Jettisoned
        fn get_Jettisoned(&self) -> Bool;

    }

    impl Flight {
        /// Simulate and return the total aerodynamic forces acting on the
        /// vessel,
        /// if it where to be traveling with the given velocity at the given
        /// position in the
        /// atmosphere of the given celestial body.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction that the force acts,
        /// with its magnitude equal to the strength of the force in Newtons.
        /// real name: Flight_SimulateAerodynamicForceAt
        fn SimulateAerodynamicForceAt(&self, body: Class, position: (Double, Double, Double,), velocity: (Double, Double, Double,)) -> (Double, Double, Double,);

        /// The current G force acting on the vessel in <math>g</math>.
        /// real name: Flight_get_GForce
        fn get_GForce(&self) -> Float;

        /// The altitude above sea level, in meters.
        /// Measured from the center of mass of the vessel.
        /// real name: Flight_get_MeanAltitude
        fn get_MeanAltitude(&self) -> Double;

        /// The altitude above the surface of the body or sea level, whichever
        /// is closer, in meters.
        /// Measured from the center of mass of the vessel.
        /// real name: Flight_get_SurfaceAltitude
        fn get_SurfaceAltitude(&self) -> Double;

        /// The altitude above the surface of the body, in meters. When over
        /// water, this is the altitude above the sea floor.
        /// Measured from the center of mass of the vessel.
        /// real name: Flight_get_BedrockAltitude
        fn get_BedrockAltitude(&self) -> Double;

        /// The elevation of the terrain under the vessel, in meters. This is
        /// the height of the terrain above sea level,
        /// and is negative when the vessel is over the sea.
        /// real name: Flight_get_Elevation
        fn get_Elevation(&self) -> Double;

        /// The <a href="https://en.wikipedia.org/wiki/Latitude">latitude</a>
        /// of the vessel for the body being orbited, in degrees.
        /// real name: Flight_get_Latitude
        fn get_Latitude(&self) -> Double;

        /// The <a href="https://en.wikipedia.org/wiki/Longitude">longitude</a>
        /// of the vessel for the body being orbited, in degrees.
        /// real name: Flight_get_Longitude
        fn get_Longitude(&self) -> Double;

        /// The velocity of the vessel, in the reference frame
        /// T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The velocity as a vector. The vector points in the direction of
        /// travel,
        /// and its magnitude is the speed of the vessel in meters per second.
        /// real name: Flight_get_Velocity
        fn get_Velocity(&self) -> (Double, Double, Double,);

        /// The speed of the vessel in meters per second,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// real name: Flight_get_Speed
        fn get_Speed(&self) -> Double;

        /// The horizontal speed of the vessel in meters per second,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// real name: Flight_get_HorizontalSpeed
        fn get_HorizontalSpeed(&self) -> Double;

        /// The vertical speed of the vessel in meters per second,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// real name: Flight_get_VerticalSpeed
        fn get_VerticalSpeed(&self) -> Double;

        /// The position of the center of mass of the vessel,
        /// in the reference frame T:SpaceCenter.ReferenceFrame
        /// # Returns
        /// 
        /// The position as a vector.
        /// real name: Flight_get_CenterOfMass
        fn get_CenterOfMass(&self) -> (Double, Double, Double,);

        /// The rotation of the vessel, in the reference frame
        /// T:SpaceCenter.ReferenceFrame
        /// # Returns
        /// 
        /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
        /// real name: Flight_get_Rotation
        fn get_Rotation(&self) -> (Double, Double, Double, Double,);

        /// The direction that the vessel is pointing in,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_Direction
        fn get_Direction(&self) -> (Double, Double, Double,);

        /// The pitch of the vessel relative to the horizon, in degrees.
        /// A value between -90° and +90°.
        /// real name: Flight_get_Pitch
        fn get_Pitch(&self) -> Float;

        /// The heading of the vessel (its angle relative to north), in degrees.
        /// A value between 0° and 360°.
        /// real name: Flight_get_Heading
        fn get_Heading(&self) -> Float;

        /// The roll of the vessel relative to the horizon, in degrees.
        /// A value between -180° and +180°.
        /// real name: Flight_get_Roll
        fn get_Roll(&self) -> Float;

        /// The prograde direction of the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_Prograde
        fn get_Prograde(&self) -> (Double, Double, Double,);

        /// The retrograde direction of the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_Retrograde
        fn get_Retrograde(&self) -> (Double, Double, Double,);

        /// The direction normal to the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_Normal
        fn get_Normal(&self) -> (Double, Double, Double,);

        /// The direction opposite to the normal of the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_AntiNormal
        fn get_AntiNormal(&self) -> (Double, Double, Double,);

        /// The radial direction of the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_Radial
        fn get_Radial(&self) -> (Double, Double, Double,);

        /// The direction opposite to the radial direction of the vessels orbit,
        /// in the reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// real name: Flight_get_AntiRadial
        fn get_AntiRadial(&self) -> (Double, Double, Double,);

        /// The current density of the atmosphere around the vessel, in
        /// <math>kg/m^3</math>.
        /// real name: Flight_get_AtmosphereDensity
        fn get_AtmosphereDensity(&self) -> Float;

        /// The dynamic pressure acting on the vessel, in Pascals. This is a
        /// measure of the
        /// strength of the aerodynamic forces. It is equal to
        /// <math>\frac{1}{2} . \mbox{air density} . \mbox{velocity}^2</math>.
        /// It is commonly denoted <math>Q</math>.
        /// real name: Flight_get_DynamicPressure
        fn get_DynamicPressure(&self) -> Float;

        /// The static atmospheric pressure at mean sea level, in Pascals.
        /// real name: Flight_get_StaticPressureAtMSL
        fn get_StaticPressureAtMSL(&self) -> Float;

        /// The static atmospheric pressure acting on the vessel, in Pascals.
        /// real name: Flight_get_StaticPressure
        fn get_StaticPressure(&self) -> Float;

        /// The total aerodynamic forces acting on the vessel,
        /// in reference frame T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction that the force acts,
        /// with its magnitude equal to the strength of the force in Newtons.
        /// real name: Flight_get_AerodynamicForce
        fn get_AerodynamicForce(&self) -> (Double, Double, Double,);

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic
        /// lift</a>
        /// currently acting on the vessel.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction that the force acts,
        /// with its magnitude equal to the strength of the force in Newtons.
        /// real name: Flight_get_Lift
        fn get_Lift(&self) -> (Double, Double, Double,);

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Aerodynamic_force">aerodynamic
        /// drag</a> currently acting on the vessel.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction of the force, with its magnitude
        /// equal to the strength of the force in Newtons.
        /// real name: Flight_get_Drag
        fn get_Drag(&self) -> (Double, Double, Double,);

        /// The speed of sound, in the atmosphere around the vessel, in
        /// <math>m/s</math>.
        /// real name: Flight_get_SpeedOfSound
        fn get_SpeedOfSound(&self) -> Float;

        /// The speed of the vessel, in multiples of the speed of sound.
        /// real name: Flight_get_Mach
        fn get_Mach(&self) -> Float;

        /// The vessels Reynolds number.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_ReynoldsNumber
        fn get_ReynoldsNumber(&self) -> Float;

        /// The <a href="https://en.wikipedia.org/wiki/True_airspeed">true air
        /// speed</a>
        /// of the vessel, in meters per second.
        /// real name: Flight_get_TrueAirSpeed
        fn get_TrueAirSpeed(&self) -> Float;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Equivalent_airspeed">equivalent
        /// air speed</a>
        /// of the vessel, in meters per second.
        /// real name: Flight_get_EquivalentAirSpeed
        fn get_EquivalentAirSpeed(&self) -> Float;

        /// An estimate of the current terminal velocity of the vessel, in
        /// meters per second.
        /// This is the speed at which the drag forces cancel out the force of
        /// gravity.
        /// real name: Flight_get_TerminalVelocity
        fn get_TerminalVelocity(&self) -> Float;

        /// The pitch angle between the orientation of the vessel and its
        /// velocity vector,
        /// in degrees.
        /// real name: Flight_get_AngleOfAttack
        fn get_AngleOfAttack(&self) -> Float;

        /// The yaw angle between the orientation of the vessel and its
        /// velocity vector, in degrees.
        /// real name: Flight_get_SideslipAngle
        fn get_SideslipAngle(&self) -> Float;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Total_air_temperature">total
        /// air temperature</a>
        /// of the atmosphere around the vessel, in Kelvin.
        /// This includes the M:SpaceCenter.Flight.StaticAirTemperature and the
        /// vessel's kinetic energy.
        /// real name: Flight_get_TotalAirTemperature
        fn get_TotalAirTemperature(&self) -> Float;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Total_air_temperature">static
        /// (ambient)
        /// temperature</a> of the atmosphere around the vessel, in Kelvin.
        /// real name: Flight_get_StaticAirTemperature
        fn get_StaticAirTemperature(&self) -> Float;

        /// The current amount of stall, between 0 and 1. A value greater than
        /// 0.005 indicates
        /// a minor stall and a value greater than 0.5 indicates a large-scale
        /// stall.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_StallFraction
        fn get_StallFraction(&self) -> Float;

        /// The coefficient of drag. This is the amount of drag produced by the
        /// vessel.
        /// It depends on air speed, air density and wing area.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_DragCoefficient
        fn get_DragCoefficient(&self) -> Float;

        /// The coefficient of lift. This is the amount of lift produced by the
        /// vessel, and
        /// depends on air speed, air density and wing area.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_LiftCoefficient
        fn get_LiftCoefficient(&self) -> Float;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Ballistic_coefficient">ballistic coefficient</a>.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_BallisticCoefficient
        fn get_BallisticCoefficient(&self) -> Float;

        /// The thrust specific fuel consumption for the jet engines on the
        /// vessel. This is a
        /// measure of the efficiency of the engines, with a lower value
        /// indicating a more
        /// efficient vessel. This value is the number of Newtons of fuel that
        /// are burned,
        /// per hour, to produce one newton of thrust.
        /// 
        /// # Remarks
        /// 
        /// Requires <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/19321-130-ferram-aerospace-research-v0159-liebe-82117/">Ferram Aerospace Research</a>.
        /// real name: Flight_get_ThrustSpecificFuelConsumption
        fn get_ThrustSpecificFuelConsumption(&self) -> Float;

    }

    impl Force {
        /// Remove the force.
        /// real name: Force_Remove
        fn Remove(&self);

        /// The part that this force is applied to.
        /// real name: Force_get_Part
        fn get_Part(&self) -> Class;

        /// The force vector, in Newtons.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction that the force acts,
        /// with its magnitude equal to the strength of the force in Newtons.
        /// real name: Force_get_ForceVector
        fn get_ForceVector(&self) -> (Double, Double, Double,);

        /// The force vector, in Newtons.
        /// 
        /// # Returns
        /// 
        /// A vector pointing in the direction that the force acts,
        /// with its magnitude equal to the strength of the force in Newtons.
        /// real name: Force_set_ForceVector
        fn set_ForceVector(&self, value: (Double, Double, Double,));

        /// The position at which the force acts, in reference frame
        /// T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// real name: Force_get_Position
        fn get_Position(&self) -> (Double, Double, Double,);

        /// The position at which the force acts, in reference frame
        /// T:SpaceCenter.ReferenceFrame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// real name: Force_set_Position
        fn set_Position(&self, value: (Double, Double, Double,));

        /// The reference frame of the force vector and position.
        /// real name: Force_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame of the force vector and position.
        /// real name: Force_set_ReferenceFrame
        fn set_ReferenceFrame(&self, value: Class);

    }

    impl Intake {
        /// The part object for this intake.
        /// real name: Intake_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the intake is open.
        /// real name: Intake_get_Open
        fn get_Open(&self) -> Bool;

        /// Whether the intake is open.
        /// real name: Intake_set_Open
        fn set_Open(&self, value: Bool);

        /// Speed of the flow into the intake, in <math>m/s</math>.
        /// real name: Intake_get_Speed
        fn get_Speed(&self) -> Float;

        /// The rate of flow into the intake, in units of resource per second.
        /// real name: Intake_get_Flow
        fn get_Flow(&self) -> Float;

        /// The area of the intake's opening, in square meters.
        /// real name: Intake_get_Area
        fn get_Area(&self) -> Float;

    }

    impl LaunchClamp {
        /// Releases the docking clamp. Has no effect if the clamp has already
        /// been released.
        /// real name: LaunchClamp_Release
        fn Release(&self);

        /// The part object for this launch clamp.
        /// real name: LaunchClamp_get_Part
        fn get_Part(&self) -> Class;

    }

    impl Leg {
        /// The part object for this landing leg.
        /// real name: Leg_get_Part
        fn get_Part(&self) -> Class;

        /// The current state of the landing leg.
        /// real name: Leg_get_State
        fn get_State(&self) -> Enumeration;

        /// Whether the leg is deployable.
        /// real name: Leg_get_Deployable
        fn get_Deployable(&self) -> Bool;

        /// Whether the landing leg is deployed.
        /// 
        /// # Remarks
        /// 
        /// Fixed landing legs are always deployed.
        /// Returns an error if you try to deploy fixed landing gear.
        /// real name: Leg_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the landing leg is deployed.
        /// 
        /// # Remarks
        /// 
        /// Fixed landing legs are always deployed.
        /// Returns an error if you try to deploy fixed landing gear.
        /// real name: Leg_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// Returns whether the leg is touching the ground.
        /// real name: Leg_get_IsGrounded
        fn get_IsGrounded(&self) -> Bool;

    }

    impl Light {
        /// The part object for this light.
        /// real name: Light_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the light is switched on.
        /// real name: Light_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the light is switched on.
        /// real name: Light_set_Active
        fn set_Active(&self, value: Bool);

        /// The color of the light, as an RGB triple.
        /// real name: Light_get_Color
        fn get_Color(&self) -> (Float, Float, Float,);

        /// The color of the light, as an RGB triple.
        /// real name: Light_set_Color
        fn set_Color(&self, value: (Float, Float, Float,));

        /// The current power usage, in units of charge per second.
        /// real name: Light_get_PowerUsage
        fn get_PowerUsage(&self) -> Float;

    }

    impl Module {
        /// Returns `true` if the module has a field with the given name.
        /// 
        /// <param name="name">Name of the field.</param>
        /// real name: Module_HasField
        fn HasField(&self, name: String) -> Bool;

        /// Returns the value of a field.
        /// 
        /// <param name="name">Name of the field.</param>
        /// real name: Module_GetField
        fn GetField(&self, name: String) -> String;

        /// Set the value of a field to the given integer number.
        /// 
        /// <param name="name">Name of the field.</param>
        /// <param name="value">Value to set.</param>
        /// real name: Module_SetFieldInt
        fn SetFieldInt(&self, name: String, value: Sint32);

        /// Set the value of a field to the given floating point number.
        /// 
        /// <param name="name">Name of the field.</param>
        /// <param name="value">Value to set.</param>
        /// real name: Module_SetFieldFloat
        fn SetFieldFloat(&self, name: String, value: Float);

        /// Set the value of a field to the given string.
        /// 
        /// <param name="name">Name of the field.</param>
        /// <param name="value">Value to set.</param>
        /// real name: Module_SetFieldString
        fn SetFieldString(&self, name: String, value: String);

        /// Set the value of a field to its original value.
        /// 
        /// <param name="name">Name of the field.</param>
        /// real name: Module_ResetField
        fn ResetField(&self, name: String);

        /// `true` if the module has an event with the given name.
        /// 
        /// <param name="name"></param>
        /// real name: Module_HasEvent
        fn HasEvent(&self, name: String) -> Bool;

        /// Trigger the named event. Equivalent to clicking the button in the
        /// right-click menu
        /// of the part.
        /// 
        /// <param name="name"></param>
        /// real name: Module_TriggerEvent
        fn TriggerEvent(&self, name: String);

        /// `true` if the part has an action with the given name.
        /// 
        /// <param name="name"></param>
        /// real name: Module_HasAction
        fn HasAction(&self, name: String) -> Bool;

        /// Set the value of an action with the given name.
        /// 
        /// <param name="name"></param>
        /// <param name="value"></param>
        /// real name: Module_SetAction
        fn SetAction(&self, name: String, value: Bool);

        /// Name of the PartModule. For example, "ModuleEngines".
        /// real name: Module_get_Name
        fn get_Name(&self) -> String;

        /// The part that contains this module.
        /// real name: Module_get_Part
        fn get_Part(&self) -> Class;

        /// The modules field names and their associated values, as a
        /// dictionary.
        /// These are the values visible in the right-click menu of the part.
        /// real name: Module_get_Fields
        fn get_Fields(&self) -> Dictionary<>;

        /// A list of the names of all of the modules events. Events are the
        /// clickable buttons
        /// visible in the right-click menu of the part.
        /// real name: Module_get_Events
        fn get_Events(&self) -> List<String>;

        /// A list of all the names of the modules actions. These are the parts
        /// actions that can
        /// be assigned to action groups in the in-game editor.
        /// real name: Module_get_Actions
        fn get_Actions(&self) -> List<String>;

    }

    impl Node {
        /// Returns the burn vector for the maneuver node.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// vector is in.
        /// Defaults to M:SpaceCenter.Vessel.OrbitalReferenceFrame.</param>
        /// # Returns
        /// 
        /// A vector whose direction is the direction of the maneuver node
        /// burn, and
        /// magnitude is the delta-v of the burn in meters per second.
        /// 
        /// 
        /// # Remarks
        /// 
        /// Does not change when executing the maneuver node. See
        /// M:SpaceCenter.Node.RemainingBurnVector.
        /// real name: Node_BurnVector
        fn BurnVector(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// Returns the remaining burn vector for the maneuver node.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// vector is in.
        /// Defaults to M:SpaceCenter.Vessel.OrbitalReferenceFrame.</param>
        /// # Returns
        /// 
        /// A vector whose direction is the direction of the maneuver node
        /// burn, and
        /// magnitude is the delta-v of the burn in meters per second.
        /// 
        /// 
        /// # Remarks
        /// 
        /// Changes as the maneuver node is executed. See
        /// M:SpaceCenter.Node.BurnVector.
        /// real name: Node_RemainingBurnVector
        fn RemainingBurnVector(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// Removes the maneuver node.
        /// real name: Node_Remove
        fn Remove(&self);

        /// The position vector of the maneuver node in the given reference
        /// frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Node_Position
        fn Position(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The direction of the maneuver nodes burn.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Node_Direction
        fn Direction(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The magnitude of the maneuver nodes delta-v in the prograde
        /// direction,
        /// in meters per second.
        /// real name: Node_get_Prograde
        fn get_Prograde(&self) -> Double;

        /// The magnitude of the maneuver nodes delta-v in the prograde
        /// direction,
        /// in meters per second.
        /// real name: Node_set_Prograde
        fn set_Prograde(&self, value: Double);

        /// The magnitude of the maneuver nodes delta-v in the normal direction,
        /// in meters per second.
        /// real name: Node_get_Normal
        fn get_Normal(&self) -> Double;

        /// The magnitude of the maneuver nodes delta-v in the normal direction,
        /// in meters per second.
        /// real name: Node_set_Normal
        fn set_Normal(&self, value: Double);

        /// The magnitude of the maneuver nodes delta-v in the radial direction,
        /// in meters per second.
        /// real name: Node_get_Radial
        fn get_Radial(&self) -> Double;

        /// The magnitude of the maneuver nodes delta-v in the radial direction,
        /// in meters per second.
        /// real name: Node_set_Radial
        fn set_Radial(&self, value: Double);

        /// The delta-v of the maneuver node, in meters per second.
        /// 
        /// # Remarks
        /// 
        /// Does not change when executing the maneuver node. See
        /// M:SpaceCenter.Node.RemainingDeltaV.
        /// real name: Node_get_DeltaV
        fn get_DeltaV(&self) -> Double;

        /// The delta-v of the maneuver node, in meters per second.
        /// 
        /// # Remarks
        /// 
        /// Does not change when executing the maneuver node. See
        /// M:SpaceCenter.Node.RemainingDeltaV.
        /// real name: Node_set_DeltaV
        fn set_DeltaV(&self, value: Double);

        /// Gets the remaining delta-v of the maneuver node, in meters per
        /// second. Changes as the
        /// node is executed. This is equivalent to the delta-v reported
        /// in-game.
        /// real name: Node_get_RemainingDeltaV
        fn get_RemainingDeltaV(&self) -> Double;

        /// The universal time at which the maneuver will occur, in seconds.
        /// real name: Node_get_UT
        fn get_UT(&self) -> Double;

        /// The universal time at which the maneuver will occur, in seconds.
        /// real name: Node_set_UT
        fn set_UT(&self, value: Double);

        /// The time until the maneuver node will be encountered, in seconds.
        /// real name: Node_get_TimeTo
        fn get_TimeTo(&self) -> Double;

        /// The orbit that results from executing the maneuver node.
        /// real name: Node_get_Orbit
        fn get_Orbit(&self) -> Class;

        /// The reference frame that is fixed relative to the maneuver node's
        /// burn.
        /// - The origin is at the position of the maneuver node.
        /// - The y-axis points in the direction of the burn.
        /// - The x-axis and z-axis point in arbitrary but fixed directions.
        /// real name: Node_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to the maneuver node, and
        /// orientated with the orbital prograde/normal/radial directions of the
        /// original orbit at the maneuver node's position.
        /// - The origin is at the position of the maneuver node.
        /// - The x-axis points in the orbital anti-radial direction of the
        /// original
        /// orbit, at the position of the maneuver node.
        /// - The y-axis points in the orbital prograde direction of the
        /// original
        /// orbit, at the position of the maneuver node.
        /// - The z-axis points in the orbital normal direction of the original
        /// orbit,
        /// at the position of the maneuver node.
        /// real name: Node_get_OrbitalReferenceFrame
        fn get_OrbitalReferenceFrame(&self) -> Class;

    }

    impl Orbit {
        /// The direction that is normal to the orbits reference plane,
        /// in the given reference frame.
        /// The reference plane is the plane from which the orbits inclination
        /// is measured.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Orbit_static_ReferencePlaneNormal
        fn ReferencePlaneNormal(referenceFrame: Class) -> (Double, Double, Double,);

        /// The direction from which the orbits longitude of ascending node is
        /// measured,
        /// in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Orbit_static_ReferencePlaneDirection
        fn ReferencePlaneDirection(referenceFrame: Class) -> (Double, Double, Double,);

        /// The mean anomaly at the given time.
        /// 
        /// <param name="ut">The universal time in seconds.</param>
        /// real name: Orbit_MeanAnomalyAtUT
        fn MeanAnomalyAtUT(&self, ut: Double) -> Double;

        /// The orbital radius at the point in the orbit given by the true
        /// anomaly.
        /// 
        /// <param name="trueAnomaly">The true anomaly.</param>
        /// real name: Orbit_RadiusAtTrueAnomaly
        fn RadiusAtTrueAnomaly(&self, trueAnomaly: Double) -> Double;

        /// The true anomaly at the given orbital radius.
        /// 
        /// <param name="radius">The orbital radius in meters.</param>
        /// real name: Orbit_TrueAnomalyAtRadius
        fn TrueAnomalyAtRadius(&self, radius: Double) -> Double;

        /// The true anomaly at the given time.
        /// 
        /// <param name="ut">The universal time in seconds.</param>
        /// real name: Orbit_TrueAnomalyAtUT
        fn TrueAnomalyAtUT(&self, ut: Double) -> Double;

        /// The universal time, in seconds, corresponding to the given true
        /// anomaly.
        /// 
        /// <param name="trueAnomaly">True anomaly.</param>
        /// real name: Orbit_UTAtTrueAnomaly
        fn UTAtTrueAnomaly(&self, trueAnomaly: Double) -> Double;

        /// The eccentric anomaly at the given universal time.
        /// 
        /// <param name="ut">The universal time, in seconds.</param>
        /// real name: Orbit_EccentricAnomalyAtUT
        fn EccentricAnomalyAtUT(&self, ut: Double) -> Double;

        /// The orbital speed at the given time, in meters per second.
        /// 
        /// <param name="time">Time from now, in seconds.</param>
        /// real name: Orbit_OrbitalSpeedAt
        fn OrbitalSpeedAt(&self, time: Double) -> Double;

        /// The orbital radius at the given time, in meters.
        /// 
        /// <param name="ut">The universal time to measure the radius
        /// at.</param>
        /// real name: Orbit_RadiusAt
        fn RadiusAt(&self, ut: Double) -> Double;

        /// The position at a given time, in the specified reference frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="ut">The universal time to measure the position
        /// at.</param>
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Orbit_PositionAt
        fn PositionAt(&self, ut: Double, referenceFrame: Class) -> (Double, Double, Double,);

        /// Estimates and returns the time at closest approach to a target
        /// orbit.
        /// 
        /// # Returns
        /// 
        /// The universal time at closest approach, in seconds.
        /// 
        /// <param name="target">Target orbit.</param>
        /// real name: Orbit_TimeOfClosestApproach
        fn TimeOfClosestApproach(&self, target: Class) -> Double;

        /// Estimates and returns the distance at closest approach to a target
        /// orbit, in meters.
        /// 
        /// <param name="target">Target orbit.</param>
        /// real name: Orbit_DistanceAtClosestApproach
        fn DistanceAtClosestApproach(&self, target: Class) -> Double;

        /// Returns the times at closest approach and corresponding distances,
        /// to a target orbit.
        /// 
        /// # Returns
        /// 
        /// 
        /// A list of two lists.
        /// The first is a list of times at closest approach, as universal
        /// times in seconds.
        /// The second is a list of corresponding distances at closest
        /// approach, in meters.
        /// 
        /// 
        /// <param name="target">Target orbit.</param>
        /// <param name="orbits">The number of future orbits to search.</param>
        /// real name: Orbit_ListClosestApproaches
        fn ListClosestApproaches(&self, target: Class, orbits: Sint32) -> List<List<Double>>;

        /// The true anomaly of the ascending node with the given target orbit.
        /// 
        /// <param name="target">Target orbit.</param>
        /// real name: Orbit_TrueAnomalyAtAN
        fn TrueAnomalyAtAN(&self, target: Class) -> Double;

        /// The true anomaly of the descending node with the given target orbit.
        /// 
        /// <param name="target">Target orbit.</param>
        /// real name: Orbit_TrueAnomalyAtDN
        fn TrueAnomalyAtDN(&self, target: Class) -> Double;

        /// Relative inclination of this orbit and the target orbit, in radians.
        /// 
        /// <param name="target">Target orbit.</param>
        /// real name: Orbit_RelativeInclination
        fn RelativeInclination(&self, target: Class) -> Double;

        /// The celestial body (e.g. planet or moon) around which the object is
        /// orbiting.
        /// real name: Orbit_get_Body
        fn get_Body(&self) -> Class;

        /// Gets the apoapsis of the orbit, in meters, from the center of mass
        /// of the body being orbited.
        /// 
        /// # Remarks
        /// 
        /// For the apoapsis altitude reported on the in-game map view,
        /// use M:SpaceCenter.Orbit.ApoapsisAltitude.
        /// real name: Orbit_get_Apoapsis
        fn get_Apoapsis(&self) -> Double;

        /// The periapsis of the orbit, in meters, from the center of mass
        /// of the body being orbited.
        /// 
        /// # Remarks
        /// 
        /// For the periapsis altitude reported on the in-game map view,
        /// use M:SpaceCenter.Orbit.PeriapsisAltitude.
        /// real name: Orbit_get_Periapsis
        fn get_Periapsis(&self) -> Double;

        /// The apoapsis of the orbit, in meters, above the sea level of the
        /// body being orbited.
        /// 
        /// # Remarks
        /// 
        /// This is equal to M:SpaceCenter.Orbit.Apoapsis minus the equatorial
        /// radius of the body.
        /// real name: Orbit_get_ApoapsisAltitude
        fn get_ApoapsisAltitude(&self) -> Double;

        /// The periapsis of the orbit, in meters, above the sea level of the
        /// body being orbited.
        /// 
        /// # Remarks
        /// 
        /// This is equal to M:SpaceCenter.Orbit.Periapsis minus the equatorial
        /// radius of the body.
        /// real name: Orbit_get_PeriapsisAltitude
        fn get_PeriapsisAltitude(&self) -> Double;

        /// The semi-major axis of the orbit, in meters.
        /// real name: Orbit_get_SemiMajorAxis
        fn get_SemiMajorAxis(&self) -> Double;

        /// The semi-minor axis of the orbit, in meters.
        /// real name: Orbit_get_SemiMinorAxis
        fn get_SemiMinorAxis(&self) -> Double;

        /// The current radius of the orbit, in meters. This is the distance
        /// between the center
        /// of mass of the object in orbit, and the center of mass of the body
        /// around which it
        /// is orbiting.
        /// 
        /// # Remarks
        /// 
        /// This value will change over time if the orbit is elliptical.
        /// real name: Orbit_get_Radius
        fn get_Radius(&self) -> Double;

        /// The current orbital speed of the object in meters per second.
        /// 
        /// # Remarks
        /// 
        /// This value will change over time if the orbit is elliptical.
        /// real name: Orbit_get_Speed
        fn get_Speed(&self) -> Double;

        /// The orbital period, in seconds.
        /// real name: Orbit_get_Period
        fn get_Period(&self) -> Double;

        /// The time until the object reaches apoapsis, in seconds.
        /// real name: Orbit_get_TimeToApoapsis
        fn get_TimeToApoapsis(&self) -> Double;

        /// The time until the object reaches periapsis, in seconds.
        /// real name: Orbit_get_TimeToPeriapsis
        fn get_TimeToPeriapsis(&self) -> Double;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Orbital_eccentricity">eccentricity</a>
        /// of the orbit.
        /// real name: Orbit_get_Eccentricity
        fn get_Eccentricity(&self) -> Double;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Orbital_inclination">inclination</a>
        /// of the orbit,
        /// in radians.
        /// real name: Orbit_get_Inclination
        fn get_Inclination(&self) -> Double;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node">longitude of
        /// the ascending node</a>, in radians.
        /// real name: Orbit_get_LongitudeOfAscendingNode
        fn get_LongitudeOfAscendingNode(&self) -> Double;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Argument_of_periapsis">argument
        /// of
        /// periapsis</a>, in radians.
        /// real name: Orbit_get_ArgumentOfPeriapsis
        fn get_ArgumentOfPeriapsis(&self) -> Double;

        /// The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean
        /// anomaly at epoch</a>.
        /// real name: Orbit_get_MeanAnomalyAtEpoch
        fn get_MeanAnomalyAtEpoch(&self) -> Double;

        /// The time since the epoch (the point at which the
        /// <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean anomaly
        /// at epoch</a>
        /// was measured, in seconds.
        /// real name: Orbit_get_Epoch
        fn get_Epoch(&self) -> Double;

        /// The <a href="https://en.wikipedia.org/wiki/Mean_anomaly">mean
        /// anomaly</a>.
        /// real name: Orbit_get_MeanAnomaly
        fn get_MeanAnomaly(&self) -> Double;

        /// The <a
        /// href="https://en.wikipedia.org/wiki/Eccentric_anomaly">eccentric
        /// anomaly</a>.
        /// real name: Orbit_get_EccentricAnomaly
        fn get_EccentricAnomaly(&self) -> Double;

        /// The <a href="https://en.wikipedia.org/wiki/True_anomaly">true
        /// anomaly</a>.
        /// real name: Orbit_get_TrueAnomaly
        fn get_TrueAnomaly(&self) -> Double;

        /// If the object is going to change sphere of influence in the future,
        /// returns the new
        /// orbit after the change. Otherwise returns `null`.
        /// real name: Orbit_get_NextOrbit
        fn get_NextOrbit(&self) -> Option<Class>;

        /// The time until the object changes sphere of influence, in seconds.
        /// Returns `NaN`
        /// if the object is not going to change sphere of influence.
        /// real name: Orbit_get_TimeToSOIChange
        fn get_TimeToSOIChange(&self) -> Double;

        /// The current orbital speed in meters per second.
        /// real name: Orbit_get_OrbitalSpeed
        fn get_OrbitalSpeed(&self) -> Double;

    }

    impl Parachute {
        /// Deploys the parachute. This has no effect if the parachute has
        /// already
        /// been deployed.
        /// real name: Parachute_Deploy
        fn Deploy(&self);

        /// Deploys the parachute. This has no effect if the parachute has
        /// already
        /// been armed or deployed. Only applicable to RealChutes parachutes.
        /// real name: Parachute_Arm
        fn Arm(&self);

        /// The part object for this parachute.
        /// real name: Parachute_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the parachute has been deployed.
        /// real name: Parachute_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the parachute has been armed or deployed. Only applicable to
        /// RealChutes parachutes.
        /// real name: Parachute_get_Armed
        fn get_Armed(&self) -> Bool;

        /// The current state of the parachute.
        /// real name: Parachute_get_State
        fn get_State(&self) -> Enumeration;

        /// The altitude at which the parachute will full deploy, in meters.
        /// Only applicable to stock parachutes.
        /// real name: Parachute_get_DeployAltitude
        fn get_DeployAltitude(&self) -> Float;

        /// The altitude at which the parachute will full deploy, in meters.
        /// Only applicable to stock parachutes.
        /// real name: Parachute_set_DeployAltitude
        fn set_DeployAltitude(&self, value: Float);

        /// The minimum pressure at which the parachute will semi-deploy, in
        /// atmospheres.
        /// Only applicable to stock parachutes.
        /// real name: Parachute_get_DeployMinPressure
        fn get_DeployMinPressure(&self) -> Float;

        /// The minimum pressure at which the parachute will semi-deploy, in
        /// atmospheres.
        /// Only applicable to stock parachutes.
        /// real name: Parachute_set_DeployMinPressure
        fn set_DeployMinPressure(&self, value: Float);

    }

    impl Part {
        /// The position of the part in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// # Remarks
        /// 
        /// This is a fixed position in the part, defined by the parts model.
        /// It s not necessarily the same as the parts center of mass.
        /// Use M:SpaceCenter.Part.CenterOfMass to get the parts center of mass.
        /// real name: Part_Position
        fn Position(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The position of the parts center of mass in the given reference
        /// frame.
        /// If the part is physicsless, this is equivalent to
        /// M:SpaceCenter.Part.Position.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Part_CenterOfMass
        fn CenterOfMass(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The axis-aligned bounding box of the part in the given reference
        /// frame.
        /// 
        /// # Returns
        /// 
        /// The positions of the minimum and maximum vertices of the box,
        /// as position vectors.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vectors are in.</param>
        /// # Remarks
        /// 
        /// This is computed from the collision mesh of the part.
        /// If the part is not collidable, the box has zero volume and is
        /// centered on
        /// the M:SpaceCenter.Part.Position of the part.
        /// real name: Part_BoundingBox
        fn BoundingBox(&self, referenceFrame: Class) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The direction the part points in, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Part_Direction
        fn Direction(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The linear velocity of the part in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The velocity as a vector. The vector points in the direction of
        /// travel,
        /// and its magnitude is the speed of the body in meters per second.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// velocity vector is in.</param>
        /// real name: Part_Velocity
        fn Velocity(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The rotation of the part, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// rotation is in.</param>
        /// real name: Part_Rotation
        fn Rotation(&self, referenceFrame: Class) -> (Double, Double, Double, Double,);

        /// Exert a constant force on the part, acting at the given position.
        /// 
        /// # Returns
        /// 
        /// An object that can be used to remove or modify the force.
        /// 
        /// <param name="force">A vector pointing in the direction that the
        /// force acts,
        /// with its magnitude equal to the strength of the force in
        /// Newtons.</param>
        /// <param name="position">The position at which the force acts, as a
        /// vector.</param>
        /// <param name="referenceFrame">The reference frame that the
        /// force and position are in.</param>
        /// real name: Part_AddForce
        fn AddForce(&self, force: (Double, Double, Double,), position: (Double, Double, Double,), referenceFrame: Class) -> Class;

        /// Exert an instantaneous force on the part, acting at the given
        /// position.
        /// 
        /// <param name="force">A vector pointing in the direction that the
        /// force acts,
        /// with its magnitude equal to the strength of the force in
        /// Newtons.</param>
        /// <param name="position">The position at which the force acts, as a
        /// vector.</param>
        /// <param name="referenceFrame">The reference frame that the
        /// force and position are in.</param>
        /// # Remarks
        /// The force is applied instantaneously in a single physics update.
        /// real name: Part_InstantaneousForce
        fn InstantaneousForce(&self, force: (Double, Double, Double,), position: (Double, Double, Double,), referenceFrame: Class);

        /// Internal name of the part, as used in
        /// <a
        /// href="https://wiki.kerbalspaceprogram.com/wiki/CFG_File_Documentation">part cfg files</a>.
        /// For example "Mark1-2Pod".
        /// real name: Part_get_Name
        fn get_Name(&self) -> String;

        /// Title of the part, as shown when the part is right clicked in-game.
        /// For example "Mk1-2 Command Pod".
        /// real name: Part_get_Title
        fn get_Title(&self) -> String;

        /// The name tag for the part. Can be set to a custom string using the
        /// in-game user interface.
        /// 
        /// # Remarks
        /// 
        /// This string is shared with
        /// <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a>
        /// if it is installed.
        /// real name: Part_get_Tag
        fn get_Tag(&self) -> String;

        /// The name tag for the part. Can be set to a custom string using the
        /// in-game user interface.
        /// 
        /// # Remarks
        /// 
        /// This string is shared with
        /// <a
        /// href="https://forum.kerbalspaceprogram.com/index.php?/topic/61827-/">kOS</a>
        /// if it is installed.
        /// real name: Part_set_Tag
        fn set_Tag(&self, value: String);

        /// Whether the part is highlighted.
        /// real name: Part_get_Highlighted
        fn get_Highlighted(&self) -> Bool;

        /// Whether the part is highlighted.
        /// real name: Part_set_Highlighted
        fn set_Highlighted(&self, value: Bool);

        /// The color used to highlight the part, as an RGB triple.
        /// real name: Part_get_HighlightColor
        fn get_HighlightColor(&self) -> (Double, Double, Double,);

        /// The color used to highlight the part, as an RGB triple.
        /// real name: Part_set_HighlightColor
        fn set_HighlightColor(&self, value: (Double, Double, Double,));

        /// The cost of the part, in units of funds.
        /// real name: Part_get_Cost
        fn get_Cost(&self) -> Double;

        /// The vessel that contains this part.
        /// real name: Part_get_Vessel
        fn get_Vessel(&self) -> Class;

        /// The parts parent. Returns `null` if the part does not have a parent.
        /// This, in combination with M:SpaceCenter.Part.Children, can be used
        /// to traverse the vessels
        /// parts tree.
        /// real name: Part_get_Parent
        fn get_Parent(&self) -> Option<Class>;

        /// The parts children. Returns an empty list if the part has no
        /// children.
        /// This, in combination with M:SpaceCenter.Part.Parent, can be used to
        /// traverse the vessels
        /// parts tree.
        /// real name: Part_get_Children
        fn get_Children(&self) -> List<Class>;

        /// Whether the part is axially attached to its parent, i.e. on the top
        /// or bottom of its parent. If the part has no parent, returns `false`.
        /// real name: Part_get_AxiallyAttached
        fn get_AxiallyAttached(&self) -> Bool;

        /// Whether the part is radially attached to its parent, i.e. on the
        /// side of its parent.
        /// If the part has no parent, returns `false`.
        /// real name: Part_get_RadiallyAttached
        fn get_RadiallyAttached(&self) -> Bool;

        /// The stage in which this part will be activated. Returns -1 if the
        /// part is not
        /// activated by staging.
        /// real name: Part_get_Stage
        fn get_Stage(&self) -> Sint32;

        /// The stage in which this part will be decoupled. Returns -1 if the
        /// part is never
        /// decoupled from the vessel.
        /// real name: Part_get_DecoupleStage
        fn get_DecoupleStage(&self) -> Sint32;

        /// Whether the part is
        /// <a
        /// href="https://wiki.kerbalspaceprogram.com/wiki/Massless_part">massless</a>.
        /// real name: Part_get_Massless
        fn get_Massless(&self) -> Bool;

        /// The current mass of the part, including resources it contains, in
        /// kilograms.
        /// Returns zero if the part is massless.
        /// real name: Part_get_Mass
        fn get_Mass(&self) -> Double;

        /// The mass of the part, not including any resources it contains, in
        /// kilograms.
        /// Returns zero if the part is massless.
        /// real name: Part_get_DryMass
        fn get_DryMass(&self) -> Double;

        /// Whether the part is shielded from the exterior of the vessel, for
        /// example by a fairing.
        /// real name: Part_get_Shielded
        fn get_Shielded(&self) -> Bool;

        /// The dynamic pressure acting on the part, in Pascals.
        /// real name: Part_get_DynamicPressure
        fn get_DynamicPressure(&self) -> Float;

        /// The impact tolerance of the part, in meters per second.
        /// real name: Part_get_ImpactTolerance
        fn get_ImpactTolerance(&self) -> Double;

        /// Temperature of the part, in Kelvin.
        /// real name: Part_get_Temperature
        fn get_Temperature(&self) -> Double;

        /// Temperature of the skin of the part, in Kelvin.
        /// real name: Part_get_SkinTemperature
        fn get_SkinTemperature(&self) -> Double;

        /// Maximum temperature that the part can survive, in Kelvin.
        /// real name: Part_get_MaxTemperature
        fn get_MaxTemperature(&self) -> Double;

        /// Maximum temperature that the skin of the part can survive, in
        /// Kelvin.
        /// real name: Part_get_MaxSkinTemperature
        fn get_MaxSkinTemperature(&self) -> Double;

        /// A measure of how much energy it takes to increase the internal
        /// temperature of the part,
        /// in Joules per Kelvin.
        /// real name: Part_get_ThermalMass
        fn get_ThermalMass(&self) -> Float;

        /// A measure of how much energy it takes to increase the skin
        /// temperature of the part,
        /// in Joules per Kelvin.
        /// real name: Part_get_ThermalSkinMass
        fn get_ThermalSkinMass(&self) -> Float;

        /// A measure of how much energy it takes to increase the temperature
        /// of the resources
        /// contained in the part, in Joules per Kelvin.
        /// real name: Part_get_ThermalResourceMass
        fn get_ThermalResourceMass(&self) -> Float;

        /// The rate at which heat energy is begin generated by the part.
        /// For example, some engines generate heat by combusting fuel.
        /// Measured in energy per unit time, or power, in Watts.
        /// A positive value means the part is gaining heat energy, and
        /// negative means it is losing
        /// heat energy.
        /// real name: Part_get_ThermalInternalFlux
        fn get_ThermalInternalFlux(&self) -> Float;

        /// The rate at which heat energy is conducting into or out of the part
        /// via contact with
        /// other parts. Measured in energy per unit time, or power, in Watts.
        /// A positive value means the part is gaining heat energy, and
        /// negative means it is
        /// losing heat energy.
        /// real name: Part_get_ThermalConductionFlux
        fn get_ThermalConductionFlux(&self) -> Float;

        /// The rate at which heat energy is convecting into or out of the part
        /// from the
        /// surrounding atmosphere. Measured in energy per unit time, or power,
        /// in Watts.
        /// A positive value means the part is gaining heat energy, and
        /// negative means it is
        /// losing heat energy.
        /// real name: Part_get_ThermalConvectionFlux
        fn get_ThermalConvectionFlux(&self) -> Float;

        /// The rate at which heat energy is radiating into or out of the part
        /// from the surrounding
        /// environment. Measured in energy per unit time, or power, in Watts.
        /// A positive value means the part is gaining heat energy, and
        /// negative means it is
        /// losing heat energy.
        /// real name: Part_get_ThermalRadiationFlux
        fn get_ThermalRadiationFlux(&self) -> Float;

        /// The rate at which heat energy is transferring between the part's
        /// skin and its internals.
        /// Measured in energy per unit time, or power, in Watts.
        /// A positive value means the part's internals are gaining heat energy,
        /// and negative means its skin is gaining heat energy.
        /// real name: Part_get_ThermalSkinToInternalFlux
        fn get_ThermalSkinToInternalFlux(&self) -> Float;

        /// A T:SpaceCenter.Resources object for the part.
        /// real name: Part_get_Resources
        fn get_Resources(&self) -> Class;

        /// Whether this part is crossfeed capable.
        /// real name: Part_get_Crossfeed
        fn get_Crossfeed(&self) -> Bool;

        /// Whether this part is a fuel line.
        /// real name: Part_get_IsFuelLine
        fn get_IsFuelLine(&self) -> Bool;

        /// The parts that are connected to this part via fuel lines, where the
        /// direction of the
        /// fuel line is into this part.
        /// real name: Part_get_FuelLinesFrom
        fn get_FuelLinesFrom(&self) -> List<Class>;

        /// The parts that are connected to this part via fuel lines, where the
        /// direction of the
        /// fuel line is out of this part.
        /// real name: Part_get_FuelLinesTo
        fn get_FuelLinesTo(&self) -> List<Class>;

        /// The modules for this part.
        /// real name: Part_get_Modules
        fn get_Modules(&self) -> List<Class>;

        /// A T:SpaceCenter.Antenna if the part is an antenna, otherwise `null`.
        /// real name: Part_get_Antenna
        fn get_Antenna(&self) -> Option<Class>;

        /// A T:SpaceCenter.CargoBay if the part is a cargo bay, otherwise
        /// `null`.
        /// real name: Part_get_CargoBay
        fn get_CargoBay(&self) -> Option<Class>;

        /// A T:SpaceCenter.ControlSurface if the part is an aerodynamic
        /// control surface,
        /// otherwise `null`.
        /// real name: Part_get_ControlSurface
        fn get_ControlSurface(&self) -> Option<Class>;

        /// A T:SpaceCenter.Decoupler if the part is a decoupler, otherwise
        /// `null`.
        /// real name: Part_get_Decoupler
        fn get_Decoupler(&self) -> Option<Class>;

        /// A T:SpaceCenter.DockingPort if the part is a docking port,
        /// otherwise `null`.
        /// real name: Part_get_DockingPort
        fn get_DockingPort(&self) -> Option<Class>;

        /// An T:SpaceCenter.Engine if the part is an engine, otherwise `null`.
        /// real name: Part_get_Engine
        fn get_Engine(&self) -> Option<Class>;

        /// An T:SpaceCenter.Experiment if the part is a science experiment,
        /// otherwise `null`.
        /// real name: Part_get_Experiment
        fn get_Experiment(&self) -> Option<Class>;

        /// A T:SpaceCenter.Fairing if the part is a fairing, otherwise `null`.
        /// real name: Part_get_Fairing
        fn get_Fairing(&self) -> Option<Class>;

        /// An T:SpaceCenter.Intake if the part is an intake, otherwise `null`.
        /// 
        /// # Remarks
        /// 
        /// This includes any part that generates thrust. This covers many
        /// different types
        /// of engine, including liquid fuel rockets, solid rocket boosters and
        /// jet engines.
        /// For RCS thrusters see T:SpaceCenter.RCS.
        /// real name: Part_get_Intake
        fn get_Intake(&self) -> Option<Class>;

        /// A T:SpaceCenter.Leg if the part is a landing leg, otherwise `null`.
        /// real name: Part_get_Leg
        fn get_Leg(&self) -> Option<Class>;

        /// A T:SpaceCenter.LaunchClamp if the part is a launch clamp,
        /// otherwise `null`.
        /// real name: Part_get_LaunchClamp
        fn get_LaunchClamp(&self) -> Option<Class>;

        /// A T:SpaceCenter.Light if the part is a light, otherwise `null`.
        /// real name: Part_get_Light
        fn get_Light(&self) -> Option<Class>;

        /// A T:SpaceCenter.Parachute if the part is a parachute, otherwise
        /// `null`.
        /// real name: Part_get_Parachute
        fn get_Parachute(&self) -> Option<Class>;

        /// A T:SpaceCenter.Radiator if the part is a radiator, otherwise
        /// `null`.
        /// real name: Part_get_Radiator
        fn get_Radiator(&self) -> Option<Class>;

        /// A T:SpaceCenter.RCS if the part is an RCS block/thruster, otherwise
        /// `null`.
        /// real name: Part_get_RCS
        fn get_RCS(&self) -> Option<Class>;

        /// A T:SpaceCenter.ReactionWheel if the part is a reaction wheel,
        /// otherwise `null`.
        /// real name: Part_get_ReactionWheel
        fn get_ReactionWheel(&self) -> Option<Class>;

        /// A T:SpaceCenter.ResourceConverter if the part is a resource
        /// converter,
        /// otherwise `null`.
        /// real name: Part_get_ResourceConverter
        fn get_ResourceConverter(&self) -> Option<Class>;

        /// A T:SpaceCenter.ResourceHarvester if the part is a resource
        /// harvester,
        /// otherwise `null`.
        /// real name: Part_get_ResourceHarvester
        fn get_ResourceHarvester(&self) -> Option<Class>;

        /// A T:SpaceCenter.Sensor if the part is a sensor, otherwise `null`.
        /// real name: Part_get_Sensor
        fn get_Sensor(&self) -> Option<Class>;

        /// A T:SpaceCenter.SolarPanel if the part is a solar panel, otherwise
        /// `null`.
        /// real name: Part_get_SolarPanel
        fn get_SolarPanel(&self) -> Option<Class>;

        /// A T:SpaceCenter.Wheel if the part is a wheel, otherwise `null`.
        /// real name: Part_get_Wheel
        fn get_Wheel(&self) -> Option<Class>;

        /// The moment of inertia of the part in <math>kg.m^2</math> around its
        /// center of mass
        /// in the parts reference frame (T:SpaceCenter.ReferenceFrame).
        /// real name: Part_get_MomentOfInertia
        fn get_MomentOfInertia(&self) -> (Double, Double, Double,);

        /// The inertia tensor of the part in the parts reference frame
        /// (T:SpaceCenter.ReferenceFrame).
        /// Returns the 3x3 matrix as a list of elements, in row-major order.
        /// real name: Part_get_InertiaTensor
        fn get_InertiaTensor(&self) -> List<Double>;

        /// The reference frame that is fixed relative to this part, and
        /// centered on a fixed
        /// position within the part, defined by the parts model.
        /// - The origin is at the position of the part, as returned by
        /// M:SpaceCenter.Part.Position.
        /// - The axes rotate with the part.
        /// - The x, y and z axis directions depend on the design of the part.
        /// 
        /// 
        /// # Remarks
        /// 
        /// For docking port parts, this reference frame is not necessarily
        /// equivalent to the
        /// reference frame for the docking port, returned by
        /// M:SpaceCenter.DockingPort.ReferenceFrame.
        /// real name: Part_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to this part, and
        /// centered on its
        /// center of mass.
        /// - The origin is at the center of mass of the part, as returned by
        /// M:SpaceCenter.Part.CenterOfMass.
        /// - The axes rotate with the part.
        /// - The x, y and z axis directions depend on the design of the part.
        /// 
        /// 
        /// # Remarks
        /// 
        /// For docking port parts, this reference frame is not necessarily
        /// equivalent to the
        /// reference frame for the docking port, returned by
        /// M:SpaceCenter.DockingPort.ReferenceFrame.
        /// real name: Part_get_CenterOfMassReferenceFrame
        fn get_CenterOfMassReferenceFrame(&self) -> Class;

    }

    impl Parts {
        /// A list of parts whose M:SpaceCenter.Part.Name is <paramref
        /// name="name.
        /// 
        /// <param name="name"></param>
        /// real name: Parts_WithName
        fn WithName(&self, name: String) -> List<Class>;

        /// A list of all parts whose M:SpaceCenter.Part.Title is <paramref
        /// name="title.
        /// 
        /// <param name="title"></param>
        /// real name: Parts_WithTitle
        fn WithTitle(&self, title: String) -> List<Class>;

        /// A list of all parts whose M:SpaceCenter.Part.Tag is <paramref
        /// name="tag.
        /// 
        /// <param name="tag"></param>
        /// real name: Parts_WithTag
        fn WithTag(&self, tag: String) -> List<Class>;

        /// A list of all parts that contain a T:SpaceCenter.Module whose
        /// M:SpaceCenter.Module.Name is <paramref name="moduleName.
        /// 
        /// <param name="moduleName"></param>
        /// real name: Parts_WithModule
        fn WithModule(&self, moduleName: String) -> List<Class>;

        /// A list of all parts that are activated in the given <paramref
        /// name="stage.
        /// 
        /// <param name="stage"></param>
        /// real name: Parts_InStage
        fn InStage(&self, stage: Sint32) -> List<Class>;

        /// A list of all parts that are decoupled in the given <paramref
        /// name="stage.
        /// 
        /// <param name="stage"></param>
        /// real name: Parts_InDecoupleStage
        fn InDecoupleStage(&self, stage: Sint32) -> List<Class>;

        /// A list of modules (combined across all parts in the vessel) whose
        /// M:SpaceCenter.Module.Name is <paramref name="moduleName.
        /// 
        /// <param name="moduleName"></param>
        /// real name: Parts_ModulesWithName
        fn ModulesWithName(&self, moduleName: String) -> List<Class>;

        /// A list of all of the vessels parts.
        /// real name: Parts_get_All
        fn get_All(&self) -> List<Class>;

        /// The vessels root part.
        /// real name: Parts_get_Root
        fn get_Root(&self) -> Class;

        /// The part from which the vessel is controlled.
        /// real name: Parts_get_Controlling
        fn get_Controlling(&self) -> Class;

        /// The part from which the vessel is controlled.
        /// real name: Parts_set_Controlling
        fn set_Controlling(&self, value: Class);

        /// A list of all antennas in the vessel.
        /// real name: Parts_get_Antennas
        fn get_Antennas(&self) -> List<Class>;

        /// A list of all control surfaces in the vessel.
        /// real name: Parts_get_ControlSurfaces
        fn get_ControlSurfaces(&self) -> List<Class>;

        /// A list of all cargo bays in the vessel.
        /// real name: Parts_get_CargoBays
        fn get_CargoBays(&self) -> List<Class>;

        /// A list of all decouplers in the vessel.
        /// real name: Parts_get_Decouplers
        fn get_Decouplers(&self) -> List<Class>;

        /// A list of all docking ports in the vessel.
        /// real name: Parts_get_DockingPorts
        fn get_DockingPorts(&self) -> List<Class>;

        /// A list of all engines in the vessel.
        /// 
        /// # Remarks
        /// 
        /// This includes any part that generates thrust. This covers many
        /// different types
        /// of engine, including liquid fuel rockets, solid rocket boosters,
        /// jet engines and
        /// RCS thrusters.
        /// real name: Parts_get_Engines
        fn get_Engines(&self) -> List<Class>;

        /// A list of all science experiments in the vessel.
        /// real name: Parts_get_Experiments
        fn get_Experiments(&self) -> List<Class>;

        /// A list of all fairings in the vessel.
        /// real name: Parts_get_Fairings
        fn get_Fairings(&self) -> List<Class>;

        /// A list of all intakes in the vessel.
        /// real name: Parts_get_Intakes
        fn get_Intakes(&self) -> List<Class>;

        /// A list of all landing legs attached to the vessel.
        /// real name: Parts_get_Legs
        fn get_Legs(&self) -> List<Class>;

        /// A list of all launch clamps attached to the vessel.
        /// real name: Parts_get_LaunchClamps
        fn get_LaunchClamps(&self) -> List<Class>;

        /// A list of all lights in the vessel.
        /// real name: Parts_get_Lights
        fn get_Lights(&self) -> List<Class>;

        /// A list of all parachutes in the vessel.
        /// real name: Parts_get_Parachutes
        fn get_Parachutes(&self) -> List<Class>;

        /// A list of all radiators in the vessel.
        /// real name: Parts_get_Radiators
        fn get_Radiators(&self) -> List<Class>;

        /// A list of all RCS blocks/thrusters in the vessel.
        /// real name: Parts_get_RCS
        fn get_RCS(&self) -> List<Class>;

        /// A list of all reaction wheels in the vessel.
        /// real name: Parts_get_ReactionWheels
        fn get_ReactionWheels(&self) -> List<Class>;

        /// A list of all resource converters in the vessel.
        /// real name: Parts_get_ResourceConverters
        fn get_ResourceConverters(&self) -> List<Class>;

        /// A list of all resource harvesters in the vessel.
        /// real name: Parts_get_ResourceHarvesters
        fn get_ResourceHarvesters(&self) -> List<Class>;

        /// A list of all sensors in the vessel.
        /// real name: Parts_get_Sensors
        fn get_Sensors(&self) -> List<Class>;

        /// A list of all solar panels in the vessel.
        /// real name: Parts_get_SolarPanels
        fn get_SolarPanels(&self) -> List<Class>;

        /// A list of all wheels in the vessel.
        /// real name: Parts_get_Wheels
        fn get_Wheels(&self) -> List<Class>;

    }

    impl Propellant {
        /// The name of the propellant.
        /// real name: Propellant_get_Name
        fn get_Name(&self) -> String;

        /// The current amount of propellant.
        /// real name: Propellant_get_CurrentAmount
        fn get_CurrentAmount(&self) -> Double;

        /// The required amount of propellant.
        /// real name: Propellant_get_CurrentRequirement
        fn get_CurrentRequirement(&self) -> Double;

        /// The total amount of the underlying resource currently reachable
        /// given
        /// resource flow rules.
        /// real name: Propellant_get_TotalResourceAvailable
        fn get_TotalResourceAvailable(&self) -> Double;

        /// The total vehicle capacity for the underlying propellant resource,
        /// restricted by resource flow rules.
        /// real name: Propellant_get_TotalResourceCapacity
        fn get_TotalResourceCapacity(&self) -> Double;

        /// If this propellant should be ignored when calculating required mass
        /// flow
        /// given specific impulse.
        /// real name: Propellant_get_IgnoreForIsp
        fn get_IgnoreForIsp(&self) -> Bool;

        /// If this propellant should be ignored for thrust curve calculations.
        /// real name: Propellant_get_IgnoreForThrustCurve
        fn get_IgnoreForThrustCurve(&self) -> Bool;

        /// If this propellant has a stack gauge or not.
        /// real name: Propellant_get_DrawStackGauge
        fn get_DrawStackGauge(&self) -> Bool;

        /// If this propellant is deprived.
        /// real name: Propellant_get_IsDeprived
        fn get_IsDeprived(&self) -> Bool;

        /// The propellant ratio.
        /// real name: Propellant_get_Ratio
        fn get_Ratio(&self) -> Float;

    }

    impl RCS {
        /// The part object for this RCS.
        /// real name: RCS_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the RCS thrusters are active.
        /// An RCS thruster is inactive if the RCS action group is disabled
        /// (M:SpaceCenter.Control.RCS), the RCS thruster itself is not enabled
        /// (M:SpaceCenter.RCS.Enabled) or it is covered by a fairing
        /// (M:SpaceCenter.Part.Shielded).
        /// real name: RCS_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the RCS thrusters are enabled.
        /// real name: RCS_get_Enabled
        fn get_Enabled(&self) -> Bool;

        /// Whether the RCS thrusters are enabled.
        /// real name: RCS_set_Enabled
        fn set_Enabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when pitch control input is
        /// given.
        /// real name: RCS_get_PitchEnabled
        fn get_PitchEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when pitch control input is
        /// given.
        /// real name: RCS_set_PitchEnabled
        fn set_PitchEnabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when yaw control input is given.
        /// real name: RCS_get_YawEnabled
        fn get_YawEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when yaw control input is given.
        /// real name: RCS_set_YawEnabled
        fn set_YawEnabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when roll control input is given.
        /// real name: RCS_get_RollEnabled
        fn get_RollEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when roll control input is given.
        /// real name: RCS_set_RollEnabled
        fn set_RollEnabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when pitch control input is
        /// given.
        /// real name: RCS_get_ForwardEnabled
        fn get_ForwardEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when pitch control input is
        /// given.
        /// real name: RCS_set_ForwardEnabled
        fn set_ForwardEnabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when yaw control input is given.
        /// real name: RCS_get_UpEnabled
        fn get_UpEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when yaw control input is given.
        /// real name: RCS_set_UpEnabled
        fn set_UpEnabled(&self, value: Bool);

        /// Whether the RCS thruster will fire when roll control input is given.
        /// real name: RCS_get_RightEnabled
        fn get_RightEnabled(&self) -> Bool;

        /// Whether the RCS thruster will fire when roll control input is given.
        /// real name: RCS_set_RightEnabled
        fn set_RightEnabled(&self, value: Bool);

        /// The available torque, in Newton meters, that can be produced by
        /// this RCS,
        /// in the positive and negative pitch, roll and yaw axes of the
        /// vessel. These axes
        /// correspond to the coordinate axes of the
        /// M:SpaceCenter.Vessel.ReferenceFrame.
        /// Returns zero if RCS is disable.
        /// real name: RCS_get_AvailableTorque
        fn get_AvailableTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum amount of thrust that can be produced by the RCS
        /// thrusters when active,
        /// in Newtons.
        /// real name: RCS_get_MaxThrust
        fn get_MaxThrust(&self) -> Float;

        /// The maximum amount of thrust that can be produced by the RCS
        /// thrusters when active
        /// in a vacuum, in Newtons.
        /// real name: RCS_get_MaxVacuumThrust
        fn get_MaxVacuumThrust(&self) -> Float;

        /// A list of thrusters, one of each nozzel in the RCS part.
        /// real name: RCS_get_Thrusters
        fn get_Thrusters(&self) -> List<Class>;

        /// The current specific impulse of the RCS, in seconds. Returns zero
        /// if the RCS is not active.
        /// real name: RCS_get_SpecificImpulse
        fn get_SpecificImpulse(&self) -> Float;

        /// The vacuum specific impulse of the RCS, in seconds.
        /// real name: RCS_get_VacuumSpecificImpulse
        fn get_VacuumSpecificImpulse(&self) -> Float;

        /// The specific impulse of the RCS at sea level on Kerbin, in seconds.
        /// real name: RCS_get_KerbinSeaLevelSpecificImpulse
        fn get_KerbinSeaLevelSpecificImpulse(&self) -> Float;

        /// The names of resources that the RCS consumes.
        /// real name: RCS_get_Propellants
        fn get_Propellants(&self) -> List<String>;

        /// The ratios of resources that the RCS consumes. A dictionary mapping
        /// resource names
        /// to the ratios at which they are consumed by the RCS.
        /// real name: RCS_get_PropellantRatios
        fn get_PropellantRatios(&self) -> Dictionary<>;

        /// Whether the RCS has fuel available.
        /// 
        /// # Remarks
        /// 
        /// The RCS thruster must be activated for this property to update
        /// correctly.
        /// real name: RCS_get_HasFuel
        fn get_HasFuel(&self) -> Bool;

    }

    impl Radiator {
        /// The part object for this radiator.
        /// real name: Radiator_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the radiator is deployable.
        /// real name: Radiator_get_Deployable
        fn get_Deployable(&self) -> Bool;

        /// For a deployable radiator, `true` if the radiator is extended.
        /// If the radiator is not deployable, this is always `true`.
        /// real name: Radiator_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// For a deployable radiator, `true` if the radiator is extended.
        /// If the radiator is not deployable, this is always `true`.
        /// real name: Radiator_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// The current state of the radiator.
        /// 
        /// # Remarks
        /// 
        /// A fixed radiator is always M:SpaceCenter.RadiatorState.Extended.
        /// real name: Radiator_get_State
        fn get_State(&self) -> Enumeration;

    }

    impl ReactionWheel {
        /// The part object for this reaction wheel.
        /// real name: ReactionWheel_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the reaction wheel is active.
        /// real name: ReactionWheel_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the reaction wheel is active.
        /// real name: ReactionWheel_set_Active
        fn set_Active(&self, value: Bool);

        /// Whether the reaction wheel is broken.
        /// real name: ReactionWheel_get_Broken
        fn get_Broken(&self) -> Bool;

        /// The available torque, in Newton meters, that can be produced by
        /// this reaction wheel,
        /// in the positive and negative pitch, roll and yaw axes of the
        /// vessel. These axes
        /// correspond to the coordinate axes of the
        /// M:SpaceCenter.Vessel.ReferenceFrame.
        /// Returns zero if the reaction wheel is inactive or broken.
        /// real name: ReactionWheel_get_AvailableTorque
        fn get_AvailableTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque, in Newton meters, that can be produced by this
        /// reaction wheel,
        /// when it is active, in the positive and negative pitch, roll and yaw
        /// axes of the vessel.
        /// These axes correspond to the coordinate axes of the
        /// M:SpaceCenter.Vessel.ReferenceFrame.
        /// real name: ReactionWheel_get_MaxTorque
        fn get_MaxTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

    }

    impl ReferenceFrame {
        /// Create a relative reference frame. This is a custom reference frame
        /// whose components offset the components of a parent reference frame.
        /// 
        /// <param name="referenceFrame">The parent reference frame on which to
        /// base this reference frame.</param>
        /// <param name="position">The offset of the position of the origin,
        /// as a position vector. Defaults to <math>(0, 0, 0)</math></param>
        /// <param name="rotation">The rotation to apply to the parent frames
        /// rotation,
        /// as a quaternion of the form <math>(x, y, z, w)</math>.
        /// Defaults to <math>(0, 0, 0, 1)</math> (i.e. no rotation)</param>
        /// <param name="velocity">The linear velocity to offset the parent
        /// frame by,
        /// as a vector pointing in the direction of travel, whose magnitude is
        /// the speed in
        /// meters per second. Defaults to <math>(0, 0, 0)</math>.</param>
        /// <param name="angularVelocity">The angular velocity to offset the
        /// parent frame by,
        /// as a vector. This vector points in the direction of the axis of
        /// rotation,
        /// and its magnitude is the speed of the rotation in radians per
        /// second.
        /// Defaults to <math>(0, 0, 0)</math>.</param>
        /// real name: ReferenceFrame_static_CreateRelative
        fn CreateRelative(referenceFrame: Class, position: (Double, Double, Double,), rotation: (Double, Double, Double, Double,), velocity: (Double, Double, Double,), angularVelocity: (Double, Double, Double,)) -> Class;

        /// Create a hybrid reference frame. This is a custom reference frame
        /// whose components inherited from other reference frames.
        /// 
        /// <param name="position">The reference frame providing the position
        /// of the origin.</param>
        /// <param name="rotation">The reference frame providing the rotation
        /// of the frame.</param>
        /// <param name="velocity">The reference frame providing the linear
        /// velocity of the frame.
        /// </param>
        /// <param name="angularVelocity">The reference frame providing the
        /// angular velocity
        /// of the frame.</param>
        /// # Remarks
        /// 
        /// The <paramref name="position reference frame is required but all
        /// other
        /// reference frames are optional. If omitted, they are set to the
        /// <paramref name="position reference frame.
        /// real name: ReferenceFrame_static_CreateHybrid
        fn CreateHybrid(position: Class, rotation: Class, velocity: Class, angularVelocity: Class) -> Class;

    }

    impl Resource {
        /// The name of the resource.
        /// real name: Resource_get_Name
        fn get_Name(&self) -> String;

        /// The part containing the resource.
        /// real name: Resource_get_Part
        fn get_Part(&self) -> Class;

        /// The total amount of the resource that can be stored in the part.
        /// real name: Resource_get_Max
        fn get_Max(&self) -> Float;

        /// The amount of the resource that is currently stored in the part.
        /// real name: Resource_get_Amount
        fn get_Amount(&self) -> Float;

        /// The density of the resource, in <math>kg/l</math>.
        /// real name: Resource_get_Density
        fn get_Density(&self) -> Float;

        /// The flow mode of the resource.
        /// real name: Resource_get_FlowMode
        fn get_FlowMode(&self) -> Enumeration;

        /// Whether use of this resource is enabled.
        /// real name: Resource_get_Enabled
        fn get_Enabled(&self) -> Bool;

        /// Whether use of this resource is enabled.
        /// real name: Resource_set_Enabled
        fn set_Enabled(&self, value: Bool);

    }

    impl ResourceConverter {
        /// True if the specified converter is active.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Active
        fn Active(&self, index: Sint32) -> Bool;

        /// The name of the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Name
        fn Name(&self, index: Sint32) -> String;

        /// Start the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Start
        fn Start(&self, index: Sint32);

        /// Stop the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Stop
        fn Stop(&self, index: Sint32);

        /// The state of the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_State
        fn State(&self, index: Sint32) -> Enumeration;

        /// Status information for the specified converter.
        /// This is the full status message shown in the in-game UI.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_StatusInfo
        fn StatusInfo(&self, index: Sint32) -> String;

        /// List of the names of resources consumed by the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Inputs
        fn Inputs(&self, index: Sint32) -> List<String>;

        /// List of the names of resources produced by the specified converter.
        /// 
        /// <param name="index">Index of the converter.</param>
        /// real name: ResourceConverter_Outputs
        fn Outputs(&self, index: Sint32) -> List<String>;

        /// The part object for this converter.
        /// real name: ResourceConverter_get_Part
        fn get_Part(&self) -> Class;

        /// The number of converters in the part.
        /// real name: ResourceConverter_get_Count
        fn get_Count(&self) -> Sint32;

        /// The thermal efficiency of the converter, as a percentage of its
        /// maximum.
        /// real name: ResourceConverter_get_ThermalEfficiency
        fn get_ThermalEfficiency(&self) -> Float;

        /// The core temperature of the converter, in Kelvin.
        /// real name: ResourceConverter_get_CoreTemperature
        fn get_CoreTemperature(&self) -> Float;

        /// The core temperature at which the converter will operate with peak
        /// efficiency, in Kelvin.
        /// real name: ResourceConverter_get_OptimumCoreTemperature
        fn get_OptimumCoreTemperature(&self) -> Float;

    }

    impl ResourceHarvester {
        /// The part object for this harvester.
        /// real name: ResourceHarvester_get_Part
        fn get_Part(&self) -> Class;

        /// The state of the harvester.
        /// real name: ResourceHarvester_get_State
        fn get_State(&self) -> Enumeration;

        /// Whether the harvester is deployed.
        /// real name: ResourceHarvester_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the harvester is deployed.
        /// real name: ResourceHarvester_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// Whether the harvester is actively drilling.
        /// real name: ResourceHarvester_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the harvester is actively drilling.
        /// real name: ResourceHarvester_set_Active
        fn set_Active(&self, value: Bool);

        /// The rate at which the drill is extracting ore, in units per second.
        /// real name: ResourceHarvester_get_ExtractionRate
        fn get_ExtractionRate(&self) -> Float;

        /// The thermal efficiency of the drill, as a percentage of its maximum.
        /// real name: ResourceHarvester_get_ThermalEfficiency
        fn get_ThermalEfficiency(&self) -> Float;

        /// The core temperature of the drill, in Kelvin.
        /// real name: ResourceHarvester_get_CoreTemperature
        fn get_CoreTemperature(&self) -> Float;

        /// The core temperature at which the drill will operate with peak
        /// efficiency, in Kelvin.
        /// real name: ResourceHarvester_get_OptimumCoreTemperature
        fn get_OptimumCoreTemperature(&self) -> Float;

    }

    impl ResourceTransfer {
        /// Start transferring a resource transfer between a pair of parts. The
        /// transfer will move
        /// at most <paramref name="maxAmount units of the resource, depending
        /// on how much of
        /// the resource is available in the source part and how much storage
        /// is available in the
        /// destination part.
        /// Use M:SpaceCenter.ResourceTransfer.Complete to check if the
        /// transfer is complete.
        /// Use M:SpaceCenter.ResourceTransfer.Amount to see how much of the
        /// resource has been transferred.
        /// 
        /// <param name="fromPart">The part to transfer to.</param>
        /// <param name="toPart">The part to transfer from.</param>
        /// <param name="resource">The name of the resource to transfer.</param>
        /// <param name="maxAmount">The maximum amount of resource to
        /// transfer.</param>
        /// real name: ResourceTransfer_static_Start
        fn Start(fromPart: Class, toPart: Class, resource: String, maxAmount: Float) -> Class;

        /// Whether the transfer has completed.
        /// real name: ResourceTransfer_get_Complete
        fn get_Complete(&self) -> Bool;

        /// The amount of the resource that has been transferred.
        /// real name: ResourceTransfer_get_Amount
        fn get_Amount(&self) -> Float;

    }

    impl Resources {
        /// All the individual resources with the given name that can be stored.
        /// real name: Resources_WithResource
        fn WithResource(&self, name: String) -> List<Class>;

        /// Check whether the named resource can be stored.
        /// 
        /// <param name="name">The name of the resource.</param>
        /// real name: Resources_HasResource
        fn HasResource(&self, name: String) -> Bool;

        /// Returns the amount of a resource that can be stored.
        /// 
        /// <param name="name">The name of the resource.</param>
        /// real name: Resources_Max
        fn Max(&self, name: String) -> Float;

        /// Returns the amount of a resource that is currently stored.
        /// 
        /// <param name="name">The name of the resource.</param>
        /// real name: Resources_Amount
        fn Amount(&self, name: String) -> Float;

        /// Returns the density of a resource, in <math>kg/l</math>.
        /// 
        /// <param name="name">The name of the resource.</param>
        /// real name: Resources_static_Density
        fn Density(name: String) -> Float;

        /// Returns the flow mode of a resource.
        /// 
        /// <param name="name">The name of the resource.</param>
        /// real name: Resources_static_FlowMode
        fn FlowMode(name: String) -> Enumeration;

        /// All the individual resources that can be stored.
        /// real name: Resources_get_All
        fn get_All(&self) -> List<Class>;

        /// A list of resource names that can be stored.
        /// real name: Resources_get_Names
        fn get_Names(&self) -> List<String>;

        /// Whether use of all the resources are enabled.
        /// 
        /// # Remarks
        /// 
        /// This is `true` if all of the resources are enabled.
        /// If any of the resources are not enabled, this is `false`.
        /// real name: Resources_get_Enabled
        fn get_Enabled(&self) -> Bool;

        /// Whether use of all the resources are enabled.
        /// 
        /// # Remarks
        /// 
        /// This is `true` if all of the resources are enabled.
        /// If any of the resources are not enabled, this is `false`.
        /// real name: Resources_set_Enabled
        fn set_Enabled(&self, value: Bool);

    }

    impl ScienceData {
        /// Data amount.
        /// real name: ScienceData_get_DataAmount
        fn get_DataAmount(&self) -> Float;

        /// Science value.
        /// real name: ScienceData_get_ScienceValue
        fn get_ScienceValue(&self) -> Float;

        /// Transmit value.
        /// real name: ScienceData_get_TransmitValue
        fn get_TransmitValue(&self) -> Float;

    }

    impl ScienceSubject {
        /// Amount of science already earned from this subject, not updated
        /// until after
        /// transmission/recovery.
        /// real name: ScienceSubject_get_Science
        fn get_Science(&self) -> Float;

        /// Total science allowable for this subject.
        /// real name: ScienceSubject_get_ScienceCap
        fn get_ScienceCap(&self) -> Float;

        /// Whether the experiment has been completed.
        /// real name: ScienceSubject_get_IsComplete
        fn get_IsComplete(&self) -> Bool;

        /// Multiply science value by this to determine data amount in mits.
        /// real name: ScienceSubject_get_DataScale
        fn get_DataScale(&self) -> Float;

        /// Diminishing value multiplier for decreasing the science value
        /// returned from repeated
        /// experiments.
        /// real name: ScienceSubject_get_ScientificValue
        fn get_ScientificValue(&self) -> Float;

        /// Multiplier for specific Celestial Body/Experiment Situation
        /// combination.
        /// real name: ScienceSubject_get_SubjectValue
        fn get_SubjectValue(&self) -> Float;

        /// Title of science subject, displayed in science archives
        /// real name: ScienceSubject_get_Title
        fn get_Title(&self) -> String;

    }

    impl Sensor {
        /// The part object for this sensor.
        /// real name: Sensor_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the sensor is active.
        /// real name: Sensor_get_Active
        fn get_Active(&self) -> Bool;

        /// Whether the sensor is active.
        /// real name: Sensor_set_Active
        fn set_Active(&self, value: Bool);

        /// The current value of the sensor.
        /// real name: Sensor_get_Value
        fn get_Value(&self) -> String;

    }

    impl SolarPanel {
        /// The part object for this solar panel.
        /// real name: SolarPanel_get_Part
        fn get_Part(&self) -> Class;

        /// Whether the solar panel is deployable.
        /// real name: SolarPanel_get_Deployable
        fn get_Deployable(&self) -> Bool;

        /// Whether the solar panel is extended.
        /// real name: SolarPanel_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the solar panel is extended.
        /// real name: SolarPanel_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// The current state of the solar panel.
        /// real name: SolarPanel_get_State
        fn get_State(&self) -> Enumeration;

        /// The current amount of energy being generated by the solar panel, in
        /// units of charge per second.
        /// real name: SolarPanel_get_EnergyFlow
        fn get_EnergyFlow(&self) -> Float;

        /// The current amount of sunlight that is incident on the solar panel,
        /// as a percentage. A value between 0 and 1.
        /// real name: SolarPanel_get_SunExposure
        fn get_SunExposure(&self) -> Float;

    }

    impl Thruster {
        /// The position at which the thruster generates thrust, in the given
        /// reference frame.
        /// For gimballed engines, this takes into account the current rotation
        /// of the gimbal.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Thruster_ThrustPosition
        fn ThrustPosition(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The direction of the force generated by the thruster, in the given
        /// reference frame.
        /// This is opposite to the direction in which the thruster expels
        /// propellant.
        /// For gimballed engines, this takes into account the current rotation
        /// of the gimbal.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Thruster_ThrustDirection
        fn ThrustDirection(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The position at which the thruster generates thrust, when the
        /// engine is in its
        /// initial position (no gimballing), in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// # Remarks
        /// 
        /// This position can move when the gimbal rotates. This is because the
        /// thrust position and
        /// gimbal position are not necessarily the same.
        /// real name: Thruster_InitialThrustPosition
        fn InitialThrustPosition(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The direction of the force generated by the thruster, when the
        /// engine is in its
        /// initial position (no gimballing), in the given reference frame.
        /// This is opposite to the direction in which the thruster expels
        /// propellant.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Thruster_InitialThrustDirection
        fn InitialThrustDirection(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// Position around which the gimbal pivots.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Thruster_GimbalPosition
        fn GimbalPosition(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The T:SpaceCenter.Part that contains this thruster.
        /// real name: Thruster_get_Part
        fn get_Part(&self) -> Class;

        /// A reference frame that is fixed relative to the thruster and
        /// orientated with
        /// its thrust direction (M:SpaceCenter.Thruster.ThrustDirection).
        /// For gimballed engines, this takes into account the current rotation
        /// of the gimbal.
        /// - 
        /// The origin is at the position of thrust for this thruster
        /// (M:SpaceCenter.Thruster.ThrustPosition).
        /// - 
        /// The axes rotate with the thrust direction.
        /// This is the direction in which the thruster expels propellant,
        /// including any gimballing.
        /// 
        /// - The y-axis points along the thrust direction.
        /// - The x-axis and z-axis are perpendicular to the thrust direction.
        /// real name: Thruster_get_ThrustReferenceFrame
        fn get_ThrustReferenceFrame(&self) -> Class;

        /// Whether the thruster is gimballed.
        /// real name: Thruster_get_Gimballed
        fn get_Gimballed(&self) -> Bool;

        /// The current gimbal angle in the pitch, roll and yaw axes, in
        /// degrees.
        /// real name: Thruster_get_GimbalAngle
        fn get_GimbalAngle(&self) -> (Double, Double, Double,);

    }

    impl Vessel {
        /// Recover the vessel.
        /// real name: Vessel_Recover
        fn Recover(&self);

        /// Returns a T:SpaceCenter.Flight object that can be used to get flight
        /// telemetry for the vessel, in the specified reference frame.
        /// 
        /// <param name="referenceFrame">
        /// Reference frame. Defaults to the vessel's surface reference frame
        /// (M:SpaceCenter.Vessel.SurfaceReferenceFrame).
        /// </param>
        /// real name: Vessel_Flight
        fn Flight(&self, referenceFrame: Class) -> Class;

        /// Returns a T:SpaceCenter.Resources object, that can used to get
        /// information about resources stored in a given <paramref name="stage.
        /// 
        /// <param name="stage">Get resources for parts that are decoupled in
        /// this stage.</param>
        /// <param name="cumulative">When `false`, returns the resources for
        /// parts
        /// decoupled in just the given stage. When `true` returns the
        /// resources decoupled in
        /// the given stage and all subsequent stages combined.</param>
        /// real name: Vessel_ResourcesInDecoupleStage
        fn ResourcesInDecoupleStage(&self, stage: Sint32, cumulative: Bool) -> Class;

        /// The position of the center of mass of the vessel, in the given
        /// reference frame.
        /// 
        /// # Returns
        /// 
        /// The position as a vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vector is in.</param>
        /// real name: Vessel_Position
        fn Position(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The axis-aligned bounding box of the vessel in the given reference
        /// frame.
        /// 
        /// # Returns
        /// 
        /// The positions of the minimum and maximum vertices of the box,
        /// as position vectors.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// position vectors are in.</param>
        /// real name: Vessel_BoundingBox
        fn BoundingBox(&self, referenceFrame: Class) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The velocity of the center of mass of the vessel, in the given
        /// reference frame.
        /// 
        /// # Returns
        /// 
        /// The velocity as a vector. The vector points in the direction of
        /// travel,
        /// and its magnitude is the speed of the body in meters per second.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// velocity vector is in.</param>
        /// real name: Vessel_Velocity
        fn Velocity(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The rotation of the vessel, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The rotation as a quaternion of the form <math>(x, y, z, w)</math>.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// rotation is in.</param>
        /// real name: Vessel_Rotation
        fn Rotation(&self, referenceFrame: Class) -> (Double, Double, Double, Double,);

        /// The direction in which the vessel is pointing, in the given
        /// reference frame.
        /// 
        /// # Returns
        /// 
        /// The direction as a unit vector.
        /// 
        /// <param name="referenceFrame">The reference frame that the returned
        /// direction is in.</param>
        /// real name: Vessel_Direction
        fn Direction(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The angular velocity of the vessel, in the given reference frame.
        /// 
        /// # Returns
        /// 
        /// The angular velocity as a vector. The magnitude of the vector is
        /// the rotational
        /// speed of the vessel, in radians per second. The direction of the
        /// vector indicates the
        /// axis of rotation, using the right-hand rule.
        /// 
        /// <param name="referenceFrame">The reference frame the returned
        /// angular velocity is in.</param>
        /// real name: Vessel_AngularVelocity
        fn AngularVelocity(&self, referenceFrame: Class) -> (Double, Double, Double,);

        /// The name of the vessel.
        /// real name: Vessel_get_Name
        fn get_Name(&self) -> String;

        /// The name of the vessel.
        /// real name: Vessel_set_Name
        fn set_Name(&self, value: String);

        /// The type of the vessel.
        /// real name: Vessel_get_Type
        fn get_Type(&self) -> Enumeration;

        /// The type of the vessel.
        /// real name: Vessel_set_Type
        fn set_Type(&self, value: Enumeration);

        /// The situation the vessel is in.
        /// real name: Vessel_get_Situation
        fn get_Situation(&self) -> Enumeration;

        /// Whether the vessel is recoverable.
        /// real name: Vessel_get_Recoverable
        fn get_Recoverable(&self) -> Bool;

        /// The mission elapsed time in seconds.
        /// real name: Vessel_get_MET
        fn get_MET(&self) -> Double;

        /// The name of the biome the vessel is currently in.
        /// real name: Vessel_get_Biome
        fn get_Biome(&self) -> String;

        /// The current orbit of the vessel.
        /// real name: Vessel_get_Orbit
        fn get_Orbit(&self) -> Class;

        /// Returns a T:SpaceCenter.Control object that can be used to
        /// manipulate
        /// the vessel's control inputs. For example, its pitch/yaw/roll
        /// controls,
        /// RCS and thrust.
        /// real name: Vessel_get_Control
        fn get_Control(&self) -> Class;

        /// Returns a T:SpaceCenter.Comms object that can be used to interact
        /// with CommNet for this vessel.
        /// real name: Vessel_get_Comms
        fn get_Comms(&self) -> Class;

        /// An T:SpaceCenter.AutoPilot object, that can be used to perform
        /// simple auto-piloting of the vessel.
        /// real name: Vessel_get_AutoPilot
        fn get_AutoPilot(&self) -> Class;

        /// The number of crew that can occupy the vessel.
        /// real name: Vessel_get_CrewCapacity
        fn get_CrewCapacity(&self) -> Sint32;

        /// The number of crew that are occupying the vessel.
        /// real name: Vessel_get_CrewCount
        fn get_CrewCount(&self) -> Sint32;

        /// The crew in the vessel.
        /// real name: Vessel_get_Crew
        fn get_Crew(&self) -> List<Class>;

        /// A T:SpaceCenter.Resources object, that can used to get information
        /// about resources stored in the vessel.
        /// real name: Vessel_get_Resources
        fn get_Resources(&self) -> Class;

        /// A T:SpaceCenter.Parts object, that can used to interact with the
        /// parts that make up this vessel.
        /// real name: Vessel_get_Parts
        fn get_Parts(&self) -> Class;

        /// The total mass of the vessel, including resources, in kg.
        /// real name: Vessel_get_Mass
        fn get_Mass(&self) -> Float;

        /// The total mass of the vessel, excluding resources, in kg.
        /// real name: Vessel_get_DryMass
        fn get_DryMass(&self) -> Float;

        /// The total thrust currently being produced by the vessel's engines,
        /// in
        /// Newtons. This is computed by summing M:SpaceCenter.Engine.Thrust for
        /// every engine in the vessel.
        /// real name: Vessel_get_Thrust
        fn get_Thrust(&self) -> Float;

        /// Gets the total available thrust that can be produced by the vessel's
        /// active engines, in Newtons. This is computed by summing
        /// M:SpaceCenter.Engine.AvailableThrust for every active engine in the
        /// vessel.
        /// real name: Vessel_get_AvailableThrust
        fn get_AvailableThrust(&self) -> Float;

        /// The total maximum thrust that can be produced by the vessel's active
        /// engines, in Newtons. This is computed by summing
        /// M:SpaceCenter.Engine.MaxThrust for every active engine.
        /// real name: Vessel_get_MaxThrust
        fn get_MaxThrust(&self) -> Float;

        /// The total maximum thrust that can be produced by the vessel's active
        /// engines when the vessel is in a vacuum, in Newtons. This is
        /// computed by
        /// summing M:SpaceCenter.Engine.MaxVacuumThrust for every active
        /// engine.
        /// real name: Vessel_get_MaxVacuumThrust
        fn get_MaxVacuumThrust(&self) -> Float;

        /// The combined specific impulse of all active engines, in seconds.
        /// This is computed using the formula
        /// <a
        /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
        /// real name: Vessel_get_SpecificImpulse
        fn get_SpecificImpulse(&self) -> Float;

        /// The combined vacuum specific impulse of all active engines, in
        /// seconds. This is computed using the formula
        /// <a
        /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
        /// real name: Vessel_get_VacuumSpecificImpulse
        fn get_VacuumSpecificImpulse(&self) -> Float;

        /// The combined specific impulse of all active engines at sea level on
        /// Kerbin, in seconds.
        /// This is computed using the formula
        /// <a
        /// href="https://wiki.kerbalspaceprogram.com/wiki/Specific_impulse#Multiple_engines">described here</a>.
        /// real name: Vessel_get_KerbinSeaLevelSpecificImpulse
        fn get_KerbinSeaLevelSpecificImpulse(&self) -> Float;

        /// The moment of inertia of the vessel around its center of mass in
        /// <math>kg.m^2</math>.
        /// The inertia values in the returned 3-tuple are around the
        /// pitch, roll and yaw directions respectively.
        /// This corresponds to the vessels reference frame
        /// (T:SpaceCenter.ReferenceFrame).
        /// real name: Vessel_get_MomentOfInertia
        fn get_MomentOfInertia(&self) -> (Double, Double, Double,);

        /// The inertia tensor of the vessel around its center of mass,
        /// in the vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// Returns the 3x3 matrix as a list of elements, in row-major order.
        /// real name: Vessel_get_InertiaTensor
        fn get_InertiaTensor(&self) -> List<Double>;

        /// The maximum torque that the vessel generates. Includes
        /// contributions from
        /// reaction wheels, RCS, gimballed engines and aerodynamic control
        /// surfaces.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableTorque
        fn get_AvailableTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque that the currently active and powered reaction
        /// wheels can generate.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableReactionWheelTorque
        fn get_AvailableReactionWheelTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque that the currently active RCS thrusters can
        /// generate.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableRCSTorque
        fn get_AvailableRCSTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque that the currently active and gimballed engines
        /// can generate.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableEngineTorque
        fn get_AvailableEngineTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque that the aerodynamic control surfaces can
        /// generate.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableControlSurfaceTorque
        fn get_AvailableControlSurfaceTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The maximum torque that parts (excluding reaction wheels, gimballed
        /// engines,
        /// RCS and control surfaces) can generate.
        /// Returns the torques in <math>N.m</math> around each of the
        /// coordinate axes of the
        /// vessels reference frame (T:SpaceCenter.ReferenceFrame).
        /// These axes are equivalent to the pitch, roll and yaw axes of the
        /// vessel.
        /// real name: Vessel_get_AvailableOtherTorque
        fn get_AvailableOtherTorque(&self) -> ((Double, Double, Double,), (Double, Double, Double,),);

        /// The reference frame that is fixed relative to the vessel,
        /// and orientated with the vessel.
        /// - The origin is at the center of mass of the vessel.
        /// - The axes rotate with the vessel.
        /// - The x-axis points out to the right of the vessel.
        /// - The y-axis points in the forward direction of the vessel.
        /// - The z-axis points out of the bottom off the vessel.
        /// real name: Vessel_get_ReferenceFrame
        fn get_ReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to the vessel,
        /// and orientated with the vessels orbital prograde/normal/radial
        /// directions.
        /// - The origin is at the center of mass of the vessel.
        /// - The axes rotate with the orbital prograde/normal/radial
        /// directions.
        /// - The x-axis points in the orbital anti-radial direction.
        /// - The y-axis points in the orbital prograde direction.
        /// - The z-axis points in the orbital normal direction.
        /// 
        /// # Remarks
        /// 
        /// Be careful not to confuse this with 'orbit' mode on the navball.
        /// real name: Vessel_get_OrbitalReferenceFrame
        fn get_OrbitalReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to the vessel,
        /// and orientated with the surface of the body being orbited.
        /// - The origin is at the center of mass of the vessel.
        /// - The axes rotate with the north and up directions on the surface
        /// of the body.
        /// - The x-axis points in the <a
        /// href="https://en.wikipedia.org/wiki/Zenith">zenith</a>
        /// direction (upwards, normal to the body being orbited, from the
        /// center of the body towards the center of
        /// mass of the vessel).
        /// - The y-axis points northwards towards the
        /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
        /// horizon</a> (north, and tangential to the
        /// surface of the body -- the direction in which a compass would point
        /// when on the surface).
        /// - The z-axis points eastwards towards the
        /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
        /// horizon</a> (east, and tangential to the
        /// surface of the body -- east on a compass when on the surface).
        /// 
        /// # Remarks
        /// 
        /// Be careful not to confuse this with 'surface' mode on the navball.
        /// real name: Vessel_get_SurfaceReferenceFrame
        fn get_SurfaceReferenceFrame(&self) -> Class;

        /// The reference frame that is fixed relative to the vessel,
        /// and orientated with the velocity vector of the vessel relative
        /// to the surface of the body being orbited.
        /// - The origin is at the center of mass of the vessel.
        /// - The axes rotate with the vessel's velocity vector.
        /// - The y-axis points in the direction of the vessel's velocity
        /// vector,
        /// relative to the surface of the body being orbited.
        /// - The z-axis is in the plane of the
        /// <a href="https://en.wikipedia.org/wiki/Horizon">astronomical
        /// horizon</a>.
        /// - The x-axis is orthogonal to the other two axes.
        /// real name: Vessel_get_SurfaceVelocityReferenceFrame
        fn get_SurfaceVelocityReferenceFrame(&self) -> Class;

    }

    impl Waypoint {
        /// Removes the waypoint.
        /// real name: Waypoint_Remove
        fn Remove(&self);

        /// The celestial body the waypoint is attached to.
        /// real name: Waypoint_get_Body
        fn get_Body(&self) -> Class;

        /// The celestial body the waypoint is attached to.
        /// real name: Waypoint_set_Body
        fn set_Body(&self, value: Class);

        /// The name of the waypoint as it appears on the map and the contract.
        /// real name: Waypoint_get_Name
        fn get_Name(&self) -> String;

        /// The name of the waypoint as it appears on the map and the contract.
        /// real name: Waypoint_set_Name
        fn set_Name(&self, value: String);

        /// The seed of the icon color. See
        /// M:SpaceCenter.WaypointManager.Colors for example colors.
        /// real name: Waypoint_get_Color
        fn get_Color(&self) -> Sint32;

        /// The seed of the icon color. See
        /// M:SpaceCenter.WaypointManager.Colors for example colors.
        /// real name: Waypoint_set_Color
        fn set_Color(&self, value: Sint32);

        /// The icon of the waypoint.
        /// real name: Waypoint_get_Icon
        fn get_Icon(&self) -> String;

        /// The icon of the waypoint.
        /// real name: Waypoint_set_Icon
        fn set_Icon(&self, value: String);

        /// The latitude of the waypoint.
        /// real name: Waypoint_get_Latitude
        fn get_Latitude(&self) -> Double;

        /// The latitude of the waypoint.
        /// real name: Waypoint_set_Latitude
        fn set_Latitude(&self, value: Double);

        /// The longitude of the waypoint.
        /// real name: Waypoint_get_Longitude
        fn get_Longitude(&self) -> Double;

        /// The longitude of the waypoint.
        /// real name: Waypoint_set_Longitude
        fn set_Longitude(&self, value: Double);

        /// The altitude of the waypoint above sea level, in meters.
        /// real name: Waypoint_get_MeanAltitude
        fn get_MeanAltitude(&self) -> Double;

        /// The altitude of the waypoint above sea level, in meters.
        /// real name: Waypoint_set_MeanAltitude
        fn set_MeanAltitude(&self, value: Double);

        /// The altitude of the waypoint above the surface of the body or sea
        /// level,
        /// whichever is closer, in meters.
        /// real name: Waypoint_get_SurfaceAltitude
        fn get_SurfaceAltitude(&self) -> Double;

        /// The altitude of the waypoint above the surface of the body or sea
        /// level,
        /// whichever is closer, in meters.
        /// real name: Waypoint_set_SurfaceAltitude
        fn set_SurfaceAltitude(&self, value: Double);

        /// The altitude of the waypoint above the surface of the body, in
        /// meters.
        /// When over water, this is the altitude above the sea floor.
        /// real name: Waypoint_get_BedrockAltitude
        fn get_BedrockAltitude(&self) -> Double;

        /// The altitude of the waypoint above the surface of the body, in
        /// meters.
        /// When over water, this is the altitude above the sea floor.
        /// real name: Waypoint_set_BedrockAltitude
        fn set_BedrockAltitude(&self, value: Double);

        /// `true` if the waypoint is near to the surface of a body.
        /// real name: Waypoint_get_NearSurface
        fn get_NearSurface(&self) -> Bool;

        /// `true` if the waypoint is attached to the ground.
        /// real name: Waypoint_get_Grounded
        fn get_Grounded(&self) -> Bool;

        /// The integer index of this waypoint within its cluster of sibling
        /// waypoints.
        /// In other words, when you have a cluster of waypoints called
        /// "Somewhere Alpha",
        /// "Somewhere Beta" and "Somewhere Gamma", the alpha site has index 0,
        /// the beta
        /// site has index 1 and the gamma site has index 2.
        /// When M:SpaceCenter.Waypoint.Clustered is `false`, this is zero.
        /// real name: Waypoint_get_Index
        fn get_Index(&self) -> Sint32;

        /// `true` if this waypoint is part of a set of clustered waypoints
        /// with greek letter
        /// names appended (Alpha, Beta, Gamma, etc).
        /// If `true`, there is a one-to-one correspondence with the greek
        /// letter name and
        /// the M:SpaceCenter.Waypoint.Index.
        /// real name: Waypoint_get_Clustered
        fn get_Clustered(&self) -> Bool;

        /// Whether the waypoint belongs to a contract.
        /// real name: Waypoint_get_HasContract
        fn get_HasContract(&self) -> Bool;

        /// The associated contract.
        /// real name: Waypoint_get_Contract
        fn get_Contract(&self) -> Class;

    }

    impl WaypointManager {
        /// Creates a waypoint at the given position at ground level, and
        /// returns a
        /// T:SpaceCenter.Waypoint object that can be used to modify it.
        /// 
        /// <param name="latitude">Latitude of the waypoint.</param>
        /// <param name="longitude">Longitude of the waypoint.</param>
        /// <param name="body">Celestial body the waypoint is attached
        /// to.</param>
        /// <param name="name">Name of the waypoint.</param>
        /// # Returns
        /// real name: WaypointManager_AddWaypoint
        fn AddWaypoint(&self, latitude: Double, longitude: Double, body: Class, name: String) -> Class;

        /// Creates a waypoint at the given position and altitude, and returns a
        /// T:SpaceCenter.Waypoint object that can be used to modify it.
        /// 
        /// <param name="latitude">Latitude of the waypoint.</param>
        /// <param name="longitude">Longitude of the waypoint.</param>
        /// <param name="altitude">Altitude (above sea level) of the
        /// waypoint.</param>
        /// <param name="body">Celestial body the waypoint is attached
        /// to.</param>
        /// <param name="name">Name of the waypoint.</param>
        /// # Returns
        /// real name: WaypointManager_AddWaypointAtAltitude
        fn AddWaypointAtAltitude(&self, latitude: Double, longitude: Double, altitude: Double, body: Class, name: String) -> Class;

        /// A list of all existing waypoints.
        /// real name: WaypointManager_get_Waypoints
        fn get_Waypoints(&self) -> List<Class>;

        /// Returns all available icons (from
        /// "GameData/Squad/Contracts/Icons/").
        /// real name: WaypointManager_get_Icons
        fn get_Icons(&self) -> List<String>;

        /// An example map of known color - seed pairs.
        /// Any other integers may be used as seed.
        /// real name: WaypointManager_get_Colors
        fn get_Colors(&self) -> Dictionary<>;

    }

    impl Wheel {
        /// The part object for this wheel.
        /// real name: Wheel_get_Part
        fn get_Part(&self) -> Class;

        /// The current state of the wheel.
        /// real name: Wheel_get_State
        fn get_State(&self) -> Enumeration;

        /// Radius of the wheel, in meters.
        /// real name: Wheel_get_Radius
        fn get_Radius(&self) -> Float;

        /// Whether the wheel is touching the ground.
        /// real name: Wheel_get_Grounded
        fn get_Grounded(&self) -> Bool;

        /// Whether the wheel has brakes.
        /// real name: Wheel_get_HasBrakes
        fn get_HasBrakes(&self) -> Bool;

        /// The braking force, as a percentage of maximum, when the brakes are
        /// applied.
        /// real name: Wheel_get_Brakes
        fn get_Brakes(&self) -> Float;

        /// The braking force, as a percentage of maximum, when the brakes are
        /// applied.
        /// real name: Wheel_set_Brakes
        fn set_Brakes(&self, value: Float);

        /// Whether automatic friction control is enabled.
        /// real name: Wheel_get_AutoFrictionControl
        fn get_AutoFrictionControl(&self) -> Bool;

        /// Whether automatic friction control is enabled.
        /// real name: Wheel_set_AutoFrictionControl
        fn set_AutoFrictionControl(&self, value: Bool);

        /// Manual friction control value. Only has an effect if automatic
        /// friction control is disabled.
        /// A value between 0 and 5 inclusive.
        /// real name: Wheel_get_ManualFrictionControl
        fn get_ManualFrictionControl(&self) -> Float;

        /// Manual friction control value. Only has an effect if automatic
        /// friction control is disabled.
        /// A value between 0 and 5 inclusive.
        /// real name: Wheel_set_ManualFrictionControl
        fn set_ManualFrictionControl(&self, value: Float);

        /// Whether the wheel is deployable.
        /// real name: Wheel_get_Deployable
        fn get_Deployable(&self) -> Bool;

        /// Whether the wheel is deployed.
        /// real name: Wheel_get_Deployed
        fn get_Deployed(&self) -> Bool;

        /// Whether the wheel is deployed.
        /// real name: Wheel_set_Deployed
        fn set_Deployed(&self, value: Bool);

        /// Whether the wheel is powered by a motor.
        /// real name: Wheel_get_Powered
        fn get_Powered(&self) -> Bool;

        /// Whether the motor is enabled.
        /// real name: Wheel_get_MotorEnabled
        fn get_MotorEnabled(&self) -> Bool;

        /// Whether the motor is enabled.
        /// real name: Wheel_set_MotorEnabled
        fn set_MotorEnabled(&self, value: Bool);

        /// Whether the direction of the motor is inverted.
        /// real name: Wheel_get_MotorInverted
        fn get_MotorInverted(&self) -> Bool;

        /// Whether the direction of the motor is inverted.
        /// real name: Wheel_set_MotorInverted
        fn set_MotorInverted(&self, value: Bool);

        /// Whether the direction of the motor is inverted.
        /// real name: Wheel_get_MotorState
        fn get_MotorState(&self) -> Enumeration;

        /// The output of the motor. This is the torque currently being
        /// generated, in Newton meters.
        /// real name: Wheel_get_MotorOutput
        fn get_MotorOutput(&self) -> Float;

        /// Whether automatic traction control is enabled.
        /// A wheel only has traction control if it is powered.
        /// real name: Wheel_get_TractionControlEnabled
        fn get_TractionControlEnabled(&self) -> Bool;

        /// Whether automatic traction control is enabled.
        /// A wheel only has traction control if it is powered.
        /// real name: Wheel_set_TractionControlEnabled
        fn set_TractionControlEnabled(&self, value: Bool);

        /// Setting for the traction control.
        /// Only takes effect if the wheel has automatic traction control
        /// enabled.
        /// A value between 0 and 5 inclusive.
        /// real name: Wheel_get_TractionControl
        fn get_TractionControl(&self) -> Float;

        /// Setting for the traction control.
        /// Only takes effect if the wheel has automatic traction control
        /// enabled.
        /// A value between 0 and 5 inclusive.
        /// real name: Wheel_set_TractionControl
        fn set_TractionControl(&self, value: Float);

        /// Manual setting for the motor limiter.
        /// Only takes effect if the wheel has automatic traction control
        /// disabled.
        /// A value between 0 and 100 inclusive.
        /// real name: Wheel_get_DriveLimiter
        fn get_DriveLimiter(&self) -> Float;

        /// Manual setting for the motor limiter.
        /// Only takes effect if the wheel has automatic traction control
        /// disabled.
        /// A value between 0 and 100 inclusive.
        /// real name: Wheel_set_DriveLimiter
        fn set_DriveLimiter(&self, value: Float);

        /// Whether the wheel has steering.
        /// real name: Wheel_get_Steerable
        fn get_Steerable(&self) -> Bool;

        /// Whether the wheel steering is enabled.
        /// real name: Wheel_get_SteeringEnabled
        fn get_SteeringEnabled(&self) -> Bool;

        /// Whether the wheel steering is enabled.
        /// real name: Wheel_set_SteeringEnabled
        fn set_SteeringEnabled(&self, value: Bool);

        /// Whether the wheel steering is inverted.
        /// real name: Wheel_get_SteeringInverted
        fn get_SteeringInverted(&self) -> Bool;

        /// Whether the wheel steering is inverted.
        /// real name: Wheel_set_SteeringInverted
        fn set_SteeringInverted(&self, value: Bool);

        /// Whether the wheel has suspension.
        /// real name: Wheel_get_HasSuspension
        fn get_HasSuspension(&self) -> Bool;

        /// Suspension spring strength, as set in the editor.
        /// real name: Wheel_get_SuspensionSpringStrength
        fn get_SuspensionSpringStrength(&self) -> Float;

        /// Suspension damper strength, as set in the editor.
        /// real name: Wheel_get_SuspensionDamperStrength
        fn get_SuspensionDamperStrength(&self) -> Float;

        /// Whether the wheel is broken.
        /// real name: Wheel_get_Broken
        fn get_Broken(&self) -> Bool;

        /// Whether the wheel is repairable.
        /// real name: Wheel_get_Repairable
        fn get_Repairable(&self) -> Bool;

        /// Current stress on the wheel.
        /// real name: Wheel_get_Stress
        fn get_Stress(&self) -> Float;

        /// Stress tolerance of the wheel.
        /// real name: Wheel_get_StressTolerance
        fn get_StressTolerance(&self) -> Float;

        /// Current stress on the wheel as a percentage of its stress tolerance.
        /// real name: Wheel_get_StressPercentage
        fn get_StressPercentage(&self) -> Float;

        /// Current deflection of the wheel.
        /// real name: Wheel_get_Deflection
        fn get_Deflection(&self) -> Float;

        /// Current slip of the wheel.
        /// real name: Wheel_get_Slip
        fn get_Slip(&self) -> Float;

    }

}

/// Provides functionality for drawing and interacting with in-game user
/// interface elements.
/// 
/// # Remarks
/// 
/// For drawing 3D objects in the flight scene, see the Drawing service.
mod UI {
    /// A text label. See M:UI.Panel.AddButton.
    struct Button;

    /// A canvas for user interface elements. See M:UI.StockCanvas and
    /// M:UI.AddCanvas.
    struct Canvas;

    /// An input field. See M:UI.Panel.AddInputField.
    struct InputField;

    /// A container for user interface elements. See M:UI.Canvas.AddPanel.
    struct Panel;

    /// A Unity engine Rect Transform for a UI object.
    /// See the <a
    /// href="https://docs.unity3d.com/Manual/class-RectTransform.html">Unity
    /// manual</a> for more details.
    struct RectTransform;

    /// A text label. See M:UI.Panel.AddText.
    struct Text;

    /// Font style.
    enum FontStyle {
        /// Normal.
        Normal = 0,
        /// Bold.
        Bold = 1,
        /// Italic.
        Italic = 2,
        /// Bold and italic.
        BoldAndItalic = 3,
    }

    /// Message position.
    enum MessagePosition {
        /// Bottom center.
        BottomCenter = 0,
        /// Top center.
        TopCenter = 1,
        /// Top left.
        TopLeft = 2,
        /// Top right.
        TopRight = 3,
    }

    /// Text alignment.
    enum TextAlignment {
        /// Left aligned.
        Left = 0,
        /// Right aligned.
        Right = 1,
        /// Center aligned.
        Center = 2,
    }

    /// Text alignment.
    enum TextAnchor {
        /// Lower center.
        LowerCenter = 0,
        /// Lower left.
        LowerLeft = 1,
        /// Lower right.
        LowerRight = 2,
        /// Middle center.
        MiddleCenter = 3,
        /// Middle left.
        MiddleLeft = 4,
        /// Middle right.
        MiddleRight = 5,
        /// Upper center.
        UpperCenter = 6,
        /// Upper left.
        UpperLeft = 7,
        /// Upper right.
        UpperRight = 8,
    }

    /// Add a new canvas.
    /// 
    /// # Remarks
    /// 
    /// If you want to add UI elements to KSPs stock UI canvas, use
    /// M:UI.StockCanvas.
    /// real name: AddCanvas
    fn AddCanvas() -> Class;

    /// Display a message on the screen.
    /// 
    /// # Remarks
    /// 
    /// The message appears just like a stock message, for example quicksave or
    /// quickload messages.
    /// 
    /// 
    /// <param name="content">Message content.</param>
    /// <param name="duration">Duration before the message disappears, in
    /// seconds.</param>
    /// <param name="position">Position to display the message.</param>
    /// <param name="size">Size of the message, differs per position.</param>
    /// <param name="color">The color of the message.</param>
    /// real name: Message
    fn Message(content: String, duration: Float, position: Enumeration, color: (Double, Double, Double,), size: Float);

    /// Remove all user interface elements.
    /// 
    /// <param name="clientOnly">If true, only remove objects created by the
    /// calling client.</param>
    /// real name: Clear
    fn Clear(clientOnly: Bool);

    /// The stock UI canvas.
    /// real name: get_StockCanvas
    fn get_StockCanvas() -> Class;

    impl Button {
        /// Remove the UI object.
        /// real name: Button_Remove
        fn Remove(&self);

        /// The rect transform for the text.
        /// real name: Button_get_RectTransform
        fn get_RectTransform(&self) -> Class;

        /// The text for the button.
        /// real name: Button_get_Text
        fn get_Text(&self) -> Class;

        /// Whether the button has been clicked.
        /// 
        /// # Remarks
        /// 
        /// This property is set to true when the user clicks the button.
        /// A client script should reset the property to false in order to
        /// detect subsequent button presses.
        /// real name: Button_get_Clicked
        fn get_Clicked(&self) -> Bool;

        /// Whether the button has been clicked.
        /// 
        /// # Remarks
        /// 
        /// This property is set to true when the user clicks the button.
        /// A client script should reset the property to false in order to
        /// detect subsequent button presses.
        /// real name: Button_set_Clicked
        fn set_Clicked(&self, value: Bool);

        /// Whether the UI object is visible.
        /// real name: Button_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the UI object is visible.
        /// real name: Button_set_Visible
        fn set_Visible(&self, value: Bool);

    }

    impl Canvas {
        /// Create a new container for user interface elements.
        /// 
        /// <param name="visible">Whether the panel is visible.</param>
        /// real name: Canvas_AddPanel
        fn AddPanel(&self, visible: Bool) -> Class;

        /// Add text to the canvas.
        /// 
        /// <param name="content">The text.</param>
        /// <param name="visible">Whether the text is visible.</param>
        /// real name: Canvas_AddText
        fn AddText(&self, content: String, visible: Bool) -> Class;

        /// Add an input field to the canvas.
        /// 
        /// <param name="visible">Whether the input field is visible.</param>
        /// real name: Canvas_AddInputField
        fn AddInputField(&self, visible: Bool) -> Class;

        /// Add a button to the canvas.
        /// 
        /// <param name="content">The label for the button.</param>
        /// <param name="visible">Whether the button is visible.</param>
        /// real name: Canvas_AddButton
        fn AddButton(&self, content: String, visible: Bool) -> Class;

        /// Remove the UI object.
        /// real name: Canvas_Remove
        fn Remove(&self);

        /// The rect transform for the canvas.
        /// real name: Canvas_get_RectTransform
        fn get_RectTransform(&self) -> Class;

        /// Whether the UI object is visible.
        /// real name: Canvas_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the UI object is visible.
        /// real name: Canvas_set_Visible
        fn set_Visible(&self, value: Bool);

    }

    impl InputField {
        /// Remove the UI object.
        /// real name: InputField_Remove
        fn Remove(&self);

        /// The rect transform for the input field.
        /// real name: InputField_get_RectTransform
        fn get_RectTransform(&self) -> Class;

        /// The value of the input field.
        /// real name: InputField_get_Value
        fn get_Value(&self) -> String;

        /// The value of the input field.
        /// real name: InputField_set_Value
        fn set_Value(&self, value: String);

        /// The text component of the input field.
        /// 
        /// # Remarks
        /// 
        /// Use M:UI.InputField.Value to get and set the value in the field.
        /// This object can be used to alter the style of the input field's
        /// text.
        /// real name: InputField_get_Text
        fn get_Text(&self) -> Class;

        /// Whether the input field has been changed.
        /// 
        /// # Remarks
        /// 
        /// This property is set to true when the user modifies the value of
        /// the input field.
        /// A client script should reset the property to false in order to
        /// detect subsequent changes.
        /// real name: InputField_get_Changed
        fn get_Changed(&self) -> Bool;

        /// Whether the input field has been changed.
        /// 
        /// # Remarks
        /// 
        /// This property is set to true when the user modifies the value of
        /// the input field.
        /// A client script should reset the property to false in order to
        /// detect subsequent changes.
        /// real name: InputField_set_Changed
        fn set_Changed(&self, value: Bool);

        /// Whether the UI object is visible.
        /// real name: InputField_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the UI object is visible.
        /// real name: InputField_set_Visible
        fn set_Visible(&self, value: Bool);

    }

    impl Panel {
        /// Create a panel within this panel.
        /// 
        /// <param name="visible">Whether the new panel is visible.</param>
        /// real name: Panel_AddPanel
        fn AddPanel(&self, visible: Bool) -> Class;

        /// Add text to the panel.
        /// 
        /// <param name="content">The text.</param>
        /// <param name="visible">Whether the text is visible.</param>
        /// real name: Panel_AddText
        fn AddText(&self, content: String, visible: Bool) -> Class;

        /// Add an input field to the panel.
        /// 
        /// <param name="visible">Whether the input field is visible.</param>
        /// real name: Panel_AddInputField
        fn AddInputField(&self, visible: Bool) -> Class;

        /// Add a button to the panel.
        /// 
        /// <param name="content">The label for the button.</param>
        /// <param name="visible">Whether the button is visible.</param>
        /// real name: Panel_AddButton
        fn AddButton(&self, content: String, visible: Bool) -> Class;

        /// Remove the UI object.
        /// real name: Panel_Remove
        fn Remove(&self);

        /// The rect transform for the panel.
        /// real name: Panel_get_RectTransform
        fn get_RectTransform(&self) -> Class;

        /// Whether the UI object is visible.
        /// real name: Panel_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the UI object is visible.
        /// real name: Panel_set_Visible
        fn set_Visible(&self, value: Bool);

    }

    impl RectTransform {
        /// Position of the rectangles pivot point relative to the anchors.
        /// real name: RectTransform_get_Position
        fn get_Position(&self) -> (Double, Double,);

        /// Position of the rectangles pivot point relative to the anchors.
        /// real name: RectTransform_set_Position
        fn set_Position(&self, value: (Double, Double,));

        /// Position of the rectangles pivot point relative to the anchors.
        /// real name: RectTransform_get_LocalPosition
        fn get_LocalPosition(&self) -> (Double, Double, Double,);

        /// Position of the rectangles pivot point relative to the anchors.
        /// real name: RectTransform_set_LocalPosition
        fn set_LocalPosition(&self, value: (Double, Double, Double,));

        /// Width and height of the rectangle.
        /// real name: RectTransform_get_Size
        fn get_Size(&self) -> (Double, Double,);

        /// Width and height of the rectangle.
        /// real name: RectTransform_set_Size
        fn set_Size(&self, value: (Double, Double,));

        /// Position of the rectangles upper right corner relative to the
        /// anchors.
        /// real name: RectTransform_get_UpperRight
        fn get_UpperRight(&self) -> (Double, Double,);

        /// Position of the rectangles upper right corner relative to the
        /// anchors.
        /// real name: RectTransform_set_UpperRight
        fn set_UpperRight(&self, value: (Double, Double,));

        /// Position of the rectangles lower left corner relative to the
        /// anchors.
        /// real name: RectTransform_get_LowerLeft
        fn get_LowerLeft(&self) -> (Double, Double,);

        /// Position of the rectangles lower left corner relative to the
        /// anchors.
        /// real name: RectTransform_set_LowerLeft
        fn set_LowerLeft(&self, value: (Double, Double,));

        /// Set the minimum and maximum anchor points as a fraction of the size
        /// of the parent rectangle.
        /// real name: RectTransform_set_Anchor
        fn set_Anchor(&self, value: (Double, Double,));

        /// The anchor point for the lower left corner of the rectangle defined
        /// as a fraction of the size of the parent rectangle.
        /// real name: RectTransform_get_AnchorMax
        fn get_AnchorMax(&self) -> (Double, Double,);

        /// The anchor point for the lower left corner of the rectangle defined
        /// as a fraction of the size of the parent rectangle.
        /// real name: RectTransform_set_AnchorMax
        fn set_AnchorMax(&self, value: (Double, Double,));

        /// The anchor point for the upper right corner of the rectangle
        /// defined as a fraction of the size of the parent rectangle.
        /// real name: RectTransform_get_AnchorMin
        fn get_AnchorMin(&self) -> (Double, Double,);

        /// The anchor point for the upper right corner of the rectangle
        /// defined as a fraction of the size of the parent rectangle.
        /// real name: RectTransform_set_AnchorMin
        fn set_AnchorMin(&self, value: (Double, Double,));

        /// Location of the pivot point around which the rectangle rotates,
        /// defined as a fraction of the size of the rectangle itself.
        /// real name: RectTransform_get_Pivot
        fn get_Pivot(&self) -> (Double, Double,);

        /// Location of the pivot point around which the rectangle rotates,
        /// defined as a fraction of the size of the rectangle itself.
        /// real name: RectTransform_set_Pivot
        fn set_Pivot(&self, value: (Double, Double,));

        /// Rotation, as a quaternion, of the object around its pivot point.
        /// real name: RectTransform_get_Rotation
        fn get_Rotation(&self) -> (Double, Double, Double, Double,);

        /// Rotation, as a quaternion, of the object around its pivot point.
        /// real name: RectTransform_set_Rotation
        fn set_Rotation(&self, value: (Double, Double, Double, Double,));

        /// Scale factor applied to the object in the x, y and z dimensions.
        /// real name: RectTransform_get_Scale
        fn get_Scale(&self) -> (Double, Double, Double,);

        /// Scale factor applied to the object in the x, y and z dimensions.
        /// real name: RectTransform_set_Scale
        fn set_Scale(&self, value: (Double, Double, Double,));

    }

    impl Text {
        /// Remove the UI object.
        /// real name: Text_Remove
        fn Remove(&self);

        /// The rect transform for the text.
        /// real name: Text_get_RectTransform
        fn get_RectTransform(&self) -> Class;

        /// A list of all available fonts.
        /// real name: Text_get_AvailableFonts
        fn get_AvailableFonts(&self) -> List<String>;

        /// The text string
        /// real name: Text_get_Content
        fn get_Content(&self) -> String;

        /// The text string
        /// real name: Text_set_Content
        fn set_Content(&self, value: String);

        /// Name of the font
        /// real name: Text_get_Font
        fn get_Font(&self) -> String;

        /// Name of the font
        /// real name: Text_set_Font
        fn set_Font(&self, value: String);

        /// Font size.
        /// real name: Text_get_Size
        fn get_Size(&self) -> Sint32;

        /// Font size.
        /// real name: Text_set_Size
        fn set_Size(&self, value: Sint32);

        /// Font style.
        /// real name: Text_get_Style
        fn get_Style(&self) -> Enumeration;

        /// Font style.
        /// real name: Text_set_Style
        fn set_Style(&self, value: Enumeration);

        /// Alignment.
        /// real name: Text_get_Alignment
        fn get_Alignment(&self) -> Enumeration;

        /// Alignment.
        /// real name: Text_set_Alignment
        fn set_Alignment(&self, value: Enumeration);

        /// Line spacing.
        /// real name: Text_get_LineSpacing
        fn get_LineSpacing(&self) -> Float;

        /// Line spacing.
        /// real name: Text_set_LineSpacing
        fn set_LineSpacing(&self, value: Float);

        /// Set the color
        /// real name: Text_get_Color
        fn get_Color(&self) -> (Double, Double, Double,);

        /// Set the color
        /// real name: Text_set_Color
        fn set_Color(&self, value: (Double, Double, Double,));

        /// Whether the UI object is visible.
        /// real name: Text_get_Visible
        fn get_Visible(&self) -> Bool;

        /// Whether the UI object is visible.
        /// real name: Text_set_Visible
        fn set_Visible(&self, value: Bool);

    }

}

